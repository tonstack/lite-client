use std::env; 
use std::error::Error;

use adnl::crypto::{KeyPair, SecretKey};
use adnl::AdnlAddress;
use ton_liteapi::layers::{UnwrapMessagesLayer, WrapErrorLayer};
use ton_liteapi::server::serve;
use ton_liteapi::tl::request::{Request, WrappedRequest};
use ton_liteapi::types::LiteError;
use ton_liteapi::tl::response::CurrentTime;
use ton_liteapi::tl::response::Response;
use tower::{make::Shared, ServiceBuilder};

async fn handler(req: WrappedRequest) -> Result<Response, LiteError> {
    println!("Received frame: {:?}", &req);
    if let Request::GetTime = req.request {
        Ok(Response::CurrentTime(CurrentTime { now: 1234 }))
    } else {
        Err(LiteError::UnexpectedMessage)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // ADNL: get private key from environment variable KEY or use default insecure one
    let private_key_hex = env::var("KEY").unwrap_or_else(|_| "f0971651aec4bb0d65ec3861c597687fda9c1e7d2ee8a93acb9a131aa9f3aee7".to_string());
    let private_key_bytes: [u8; 32] = hex::decode(private_key_hex)?.try_into().unwrap();
    let keypair = KeyPair::from(&SecretKey::from_bytes(private_key_bytes));

    // ADNL: print public key and adnl address associated with given private key
    println!(
        "Public key is: {}",
        hex::encode(keypair.public_key.as_bytes())
    );
    println!(
        "Address is: {}",
        hex::encode(AdnlAddress::from(&keypair.public_key).as_bytes())
    );

    let service = ServiceBuilder::new()
       .buffer(100)
       .layer(UnwrapMessagesLayer)
       .layer(WrapErrorLayer)
       .service_fn(handler);

    serve(&("127.0.0.1", 8080), keypair, Shared::new(service)).await?;
    Ok(())
}