use adnl::AdnlError;
use thiserror::Error;
use tl_proto::TlError;
use tower::Service;

use crate::tl::{request::{Request, WrappedRequest}, response::Response};

#[derive(Debug, Error)]
pub enum LiteError {
    #[error("Liteserver error with code {code}: {message}")]
    ServerError { code: i32, message: String },
    #[error("TL parsing error")]
    TlError(TlError),
    #[error("Unexpected TL message")]
    UnexpectedMessage,
    #[error("ADNL error")]
    AdnlError(AdnlError),
    #[error("IO error")]
    IoError(std::io::Error),
}

pub trait LiteService: Service<WrappedRequest, Response = Response, Error = LiteError>{}