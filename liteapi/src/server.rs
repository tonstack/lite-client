use std::{convert::Infallible, error::Error, future::poll_fn, net::SocketAddr, time::Duration};

use adnl::{AdnlError, AdnlPeer, AdnlPrivateKey};
use log::error;
use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};
use tower::{Service, MakeService, ServiceExt};

use crate::{tl::{adnl::Message, request::{Request, WrappedRequest}, response::Response}, types::{LiteError, LiteService}};

pub struct Connection<T: AsyncReadExt + AsyncWriteExt> {
    peer: AdnlPeer<T>
}

impl<T: AsyncReadExt + AsyncWriteExt + Unpin> Connection<T> {
    pub fn from_adnl(peer: AdnlPeer<T>) -> Self {
        Self { peer }
    }

    pub async fn from_transport<S>(transport: T, server_private: &S) -> Result<Self, AdnlError> where S: AdnlPrivateKey {
        Ok(Self { peer: AdnlPeer::handle_handshake(transport, server_private).await? })
    }
}

fn is_connection_error(e: &io::Error) -> bool {
    matches!(
        e.kind(),
        io::ErrorKind::ConnectionRefused
            | io::ErrorKind::ConnectionAborted
            | io::ErrorKind::ConnectionReset
    )
}

async fn tcp_accept(listener: &TcpListener) -> Option<(TcpStream, SocketAddr)> {
    match listener.accept().await {
        Ok(conn) => Some(conn),
        Err(e) => {
            if is_connection_error(&e) {
                return None;
            }

            // [From `hyper::Server` in 0.14](https://github.com/hyperium/hyper/blob/v0.14.27/src/server/tcp.rs#L186)
            //
            // > A possible scenario is that the process has hit the max open files
            // > allowed, and so trying to accept a new connection will fail with
            // > `EMFILE`. In some cases, it's preferable to just wait for some time, if
            // > the application will likely close some files (or connections), and try
            // > to accept the connection again. If this option is `true`, the error
            // > will be logged at the `error` level, since it is still a big deal,
            // > and then the listener will sleep for 1 second.
            //
            // hyper allowed customizing this but axum does not.
            error!("accept error: {e}");
            tokio::time::sleep(Duration::from_secs(1)).await;
            None
        }
    }
}