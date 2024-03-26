
use std::future::poll_fn;
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::time::Duration;

use adnl::AdnlPeer;
use adnl::AdnlPrivateKey;
use log::error;
use tokio::net::TcpListener;
use tokio::net::ToSocketAddrs;
use tokio_tower::multiplex::Server;
use tower::MakeService;
use tower::Service;

use crate::peer::LitePeer;
use crate::tl::adnl::Message;
use crate::types::LiteError;

pub async fn serve<A, S, M>(addr: &A, private_key: S, mut service_maker: M) -> Result<(), Box<dyn std::error::Error>> 
    where A: ToSocketAddrs, 
          M: MakeService<SocketAddr, Message, Response = Message> + Send,
          M::Error: std::fmt::Debug,
          M::MakeError: std::error::Error,
          M::Service: Send + 'static,
          <M::Service as Service<Message>>::Future: Send,
          S: AdnlPrivateKey + Clone + Send + Sync + 'static {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, addr) = match listener.accept().await {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => {
                // [From `hyper::Server` in 0.14](https://github.com/hyperium/hyper/blob/v0.14.27/src/server/tcp.rs#L186)
                //
                // > A possible scenario is that the process has hit the max open files
                // > allowed, and so trying to accept a new connection will fail with
                // > `EMFILE`. In some cases, it's preferable to just wait for some time, if
                // > the application will likely close some files (or connections), and try
                // > to accept the connection again. If this option is `true`, the error
                // > will be logged at the `error` level, since it is still a big deal,
                // > and then the listener will sleep for 1 second.
                if !matches!(e.kind(), ErrorKind::ConnectionRefused | ErrorKind::ConnectionAborted | ErrorKind::ConnectionReset) {
                    error!("accept error: {e}");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                continue;
            }
        };
        poll_fn(|cx| service_maker.poll_ready(cx)).await.expect("polling service maker failed");
        let service = service_maker.make_service(addr).await.expect("making service failed");
        let private_key = private_key.clone();
        tokio::spawn(async move {
            let adnl = AdnlPeer::handle_handshake(socket, &private_key).await.expect("handshake failed");
            let lite = LitePeer::new(adnl);
            Server::new(lite, service).await.expect("server failed");
        });
    }
}