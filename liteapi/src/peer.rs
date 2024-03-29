use std::task::Poll;

use adnl::AdnlError;
use futures::{Sink, Stream};
use pin_project::pin_project;
use tokio_tower::multiplex::TagStore;
use tokio_util::bytes::Bytes;

use crate::{tl::{adnl::Message, common::Int256}, types::LiteError};

#[pin_project]
pub struct LitePeer<T> {
    #[pin]
    inner: T,
}

impl<T> LitePeer<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> Sink<Message> for LitePeer<T> where T: Sink<Bytes, Error = AdnlError> {
    type Error = LiteError;
    
    fn poll_ready(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx).map_err(|e| LiteError::AdnlError(e.into()))
    }
    
    fn start_send(self: std::pin::Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        log::debug!("Sending TL message: {:?}", item);
        let data = tl_proto::serialize(item).into();
        self.project().inner.start_send(data).map_err(|e| LiteError::AdnlError(e.into()))
    }
    
    fn poll_flush(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx).map_err(|e| LiteError::AdnlError(e.into()))
    }
    
    fn poll_close(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx).map_err(|e| LiteError::AdnlError(e.into()))
    }
}

impl<T> Stream for LitePeer<T> where T: Stream<Item = Result<Bytes, AdnlError>> {
    type Item = Result<Message, LiteError>;

    fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        let inner = self.project().inner.poll_next(cx);
        match inner {
            Poll::Ready(Some(Ok(bytes))) => {
                let decoded = tl_proto::deserialize(&bytes);
                log::debug!("Decoded to TL message:\n{:?}\n{:?}", bytes, decoded);
                Poll::Ready(Some(decoded.map_err(|e| LiteError::TlError(e))))
            },
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(LiteError::AdnlError(e.into())))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<T> TagStore<Message, Message> for LitePeer<T> {
    type Tag = Int256;

    fn assign_tag(self: std::pin::Pin<&mut Self>, r: &mut Message) -> Self::Tag {
        match r {
            Message::Answer { query_id, .. } => { *query_id = Int256::random(); query_id.clone() },
            Message::Query { query_id, .. } => { *query_id = Int256::random(); query_id.clone() },
        }
    }

    fn finish_tag(self: std::pin::Pin<&mut Self>, r: &Message) -> Self::Tag {
        match r {
            Message::Answer { query_id, .. } => query_id.clone(),
            Message::Query { query_id, .. } => query_id.clone(),
        }
    }
}