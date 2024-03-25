use std::{env, error::Error};

use adnl::{AdnlPeer, AdnlPrivateKey, AdnlPublicKey};
use tokio::net::TcpListener;
use tokio_tower::pipeline::Server;
use ton_liteapi::{peer::LitePeer, tl::{adnl::Message, response}, types::LiteError};
use x25519_dalek::StaticSecret;

async fn handler(req: Message) -> Result<Message, LiteError> {
    let (query_id, req) = match req {
        Message::Query { query_id, query } => (query_id, query),
        _ => return Err(LiteError::UnexpectedMessage)
    };
    println!("Received frame: {:?}, tag = {}", &req, query_id);

    let response = Message::Answer { query_id, answer: response::Response::CurrentTime { now: 1234 } };
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ADNL: get private key from environment variable KEY or use default insecure one
    let private_key_hex = env::var("KEY").unwrap_or_else(|_| "f0971651aec4bb0d65ec3861c597687fda9c1e7d2ee8a93acb9a131aa9f3aee7".to_string());
    let private_key_bytes: [u8; 32] = hex::decode(private_key_hex)?.try_into().unwrap();
    let private_key = StaticSecret::from(private_key_bytes);

    let listener = TcpListener::bind(&"127.0.0.1:8080").await?;

    // ADNL: print public key and adnl address associated with given private key
    println!("Public key is: {}", hex::encode(private_key.public().edwards_repr()));
    println!("Address is: {}", hex::encode(private_key.public().address().as_bytes()));

    loop {
        let (socket, _) = listener.accept().await?;
        let private_key = private_key.clone();
        tokio::spawn(async move {
            // ADNL: handle handshake
            let adnl = AdnlPeer::handle_handshake(socket, &private_key).await.expect("handshake failed");
            // liteapi: wrap raw bytes into lite messages and spawn handler for them
            let lite = LitePeer::new(adnl);
            Server::new(lite, tower::service_fn(handler)).await.expect("server failed");
        });
    }
}