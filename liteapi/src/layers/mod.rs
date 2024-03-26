use std::task::{Context, Poll};

use futures::future::{self, BoxFuture};
use futures::FutureExt;
use tower::{Layer, Service};

use crate::tl::common::Int256;
use crate::tl::request::LiteQuery;
use crate::tl::response::Error;
use crate::{tl::{adnl::Message, request::WrappedRequest, response::Response}, types::LiteError};

pub struct WrapMessagesLayer;

impl<S> Layer<S> for WrapMessagesLayer {
    type Service = WrapService<S>;

    fn layer(&self, service: S) -> Self::Service {
        WrapService {
            service
        }
    }
}

pub struct WrapService<S> {
    service: S,
}

impl<S> Service<WrappedRequest> for WrapService<S>
where
    S: Service<Message>,
    S::Error: Into<LiteError>,
    S::Response: Into<Message>,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = LiteError;
    type Future = BoxFuture<'static, Result<Response, LiteError>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: WrappedRequest) -> Self::Future {
        let fut = self.service.call(Message::Query {
            query_id: Int256::default(), 
            query: LiteQuery {
                wrapped_request: request,
            }
        });
        Box::pin(async move {
            let response = fut.await.map_err(Into::into)?.into();

            match response {
                Message::Answer { answer, .. } => Ok(answer),
                _ => Err(LiteError::UnexpectedMessage)
            }
        })
    }
}

pub struct UnwrapMessagesLayer;

impl<S> Layer<S> for UnwrapMessagesLayer {
    type Service = UnwrapService<S>;

    fn layer(&self, service: S) -> Self::Service {
        UnwrapService {
            service
        }
    }
}

pub struct UnwrapService<S> {
    service: S,
}

impl<S> Service<Message> for UnwrapService<S>
where
    S: Service<WrappedRequest>,
    S::Error: Into<LiteError>,
    S::Response: Into<Response>,
    S::Future: Send + 'static,
{
    type Response = Message;
    type Error = LiteError;
    type Future = BoxFuture<'static, Result<Message, LiteError>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Message) -> Self::Future {
        let request = match request {
            Message::Query { query: LiteQuery { wrapped_request }, .. } => wrapped_request,
            _ => return Box::pin(future::err(LiteError::UnexpectedMessage))
        };
        let fut = self.service.call(request);
        Box::pin(async move {
            let response = fut.await.map_err(Into::into)?.into();
            Ok(Message::Answer { query_id: Int256::default(), answer: response })
        })
    }
}

pub struct WrapErrorLayer;

impl<S> Layer<S> for WrapErrorLayer {
    type Service = WrapErrorService<S>;

    fn layer(&self, service: S) -> Self::Service {
        WrapErrorService {
            service
        }
    }
}

pub struct WrapErrorService<S> {
    service: S,
}

impl<S> Service<Message> for WrapErrorService<S>
where
    S: Service<Message>,
    S::Error: Into<LiteError>,
    S::Response: Into<Message>,
    S::Future: Send + 'static,
{
    type Response = Message;
    type Error = LiteError;
    type Future = BoxFuture<'static, Result<Message, LiteError>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Message) -> Self::Future {
        let query_id = match &request {
            Message::Query { query_id, .. } => query_id,
            Message::Answer { query_id, .. } => query_id,
        }.clone();
        let fut = self.service.call(request);
        Box::pin(async move {
            let response = fut.await;
            match response {
                Ok(x) => Ok(x.into()),
                Err(e) => Ok(Message::Answer {
                    query_id,
                    answer: Response::Error(Error {
                        code: 500,
                        message: format!("{:?}", e.into()).as_str().into(),
                    })
                })
            }
        })
    }
}