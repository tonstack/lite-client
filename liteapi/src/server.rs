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

async fn handle_request<LS: LiteService, T: AsyncReadExt + AsyncWriteExt + Unpin>(service: &mut LS, peer: &mut AdnlPeer<T>) -> Result<(), Box<dyn Error>> {
    let mut buffer = Vec::with_capacity(8192);
    let size = peer.receive(&mut buffer).await?;
    let message = tl_proto::deserialize::<Message>(&buffer[..size])?;
    if let Message::Query { query_id, query } = message {
        let response = service.ready().await?.call(query.wrapped_request).await?;
        let answer = Message::Answer { query_id, answer: response };
        peer.send(&mut tl_proto::serialize(answer)).await?;
        Ok(())
    }
    else {
        Err(LiteError::UnexpectedMessage.into())
    }
}

pub async fn serve<M, S>(listener: TcpListener, make_service: M, private_key: &S) -> Result<(), Box<dyn Error>> where M: MakeService<SocketAddr, WrappedRequest>, M::Service: LiteService, S: AdnlPrivateKey {
    loop {
        let (socket, remote_addr) = match tcp_accept(&listener).await {
            Some(x) => x,
            None => continue,
        };
        poll_fn(|cx| make_service.poll_ready(cx)).await?;

        let service = match make_service.make_service(remote_addr).await {
            Ok(service) => service,
            Err(_) => continue,
        };

        tokio::spawn(async move {
            let mut peer = AdnlPeer::handle_handshake(socket, private_key).await?;
            loop {
                handle_request(&mut service, &mut peer).await;
            }
        });
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