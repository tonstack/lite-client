use adnl::AdnlError;
use thiserror::Error;
use tl_proto::TlError;
use tower::Service;

use crate::tl::{request::WrappedRequest, response::Response};

#[derive(Debug, Error)]
pub enum LiteError {
    #[error("Liteserver error")]
    ServerError(crate::tl::response::Error),
    #[error("TL parsing error")]
    TlError(TlError),
    #[error("Unexpected TL message")]
    UnexpectedMessage,
    #[error("ADNL error")]
    AdnlError(#[from] AdnlError),
    #[error("Unknown error")]
    UnknownError(#[from] Box<dyn std::error::Error + Send + Sync + 'static>)
}

pub trait LiteService: Service<WrappedRequest, Response = Response, Error = LiteError> where Self::Future: Send + 'static {}

impl<T> LiteService for T where T: Service<WrappedRequest, Response = Response, Error = LiteError>, T::Future: Send + 'static {}