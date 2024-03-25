use adnl::{AdnlBuilder, AdnlPeer, AdnlPrivateKey, AdnlPublicKey};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::ToSocketAddrs};

use crate::{tl::{adnl::Message, common::Int256, request::{LiteQuery, WrappedRequest}, response::Response}, types::LiteError};

pub struct SingleClient<T: AsyncReadExt + AsyncWriteExt> {
    peer: AdnlPeer<T>
}

impl<T: AsyncReadExt + AsyncWriteExt + Unpin> SingleClient<T> {
    pub fn from_adnl(peer: AdnlPeer<T>) -> Self {
        Self { peer }
    }

    pub async fn connect_over_transport<S, P>(transport: T, client_private: &S, server_public: &P) -> Result<Self, LiteError> where S: AdnlPrivateKey, P: AdnlPublicKey {
        let peer = AdnlBuilder::with_random_aes_params(&mut rand::rngs::OsRng)
            .perform_ecdh(client_private, server_public)
            .perform_handshake(transport)
            .await
            .map_err(|e| LiteError::AdnlError(e))?;
        Ok(Self::from_adnl(peer))
    }

    pub async fn query(&mut self, request: WrappedRequest) -> Result<Response, LiteError> {
        let query_id = Int256::random();
        let query = Message::Query { query_id: query_id.clone(), query: LiteQuery { wrapped_request: request } };
        self.peer.send(&mut tl_proto::serialize(query)).await.map_err(|e| LiteError::AdnlError(e))?;
        let mut answer = Vec::with_capacity(8192);
        self.peer.receive(&mut answer).await.map_err(|e| LiteError::AdnlError(e))?;
        let message = tl_proto::deserialize::<Message>(&answer).map_err(|e| LiteError::TlError(e))?;
        if let Message::Answer { answer: response, query_id: recv_query_id } = message {
            if query_id != recv_query_id {
                return Err(LiteError::UnexpectedMessage);
            }
            Ok(response)
        } else {
            Err(LiteError::UnexpectedMessage)
        }
    }
}

impl SingleClient<tokio::net::TcpStream> {
    pub async fn connect<A, P, S>(addr: A, client_private: &S, server_public: &P) -> Result<Self, LiteError> where A: ToSocketAddrs, P: AdnlPublicKey, S: AdnlPrivateKey {
        let tcp = tokio::net::TcpStream::connect(addr).await.map_err(|e| LiteError::IoError(e))?;
        Self::connect_over_transport(tcp, client_private, server_public).await
    }
}