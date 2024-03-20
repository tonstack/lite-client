use adnl::AdnlRawPublicKey;
use ton_liteapi::{client::SingleClient, tl::request::{Request, WrappedRequest}};
use x25519_dalek::StaticSecret;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // decode liteserver public key
    let server_public = AdnlRawPublicKey::try_from(&*base64::decode("JhXt7H1dZTgxQTIyGiYV4f9VUARuDxFl/1kVBjLSMB8=")?)?;

    let ls_ip = "65.21.74.140";
    let ls_port = 46427;
    let mut client = SingleClient::connect((ls_ip, ls_port), &StaticSecret::random_from_rng(rand::thread_rng()), &server_public).await?;
    let result = client.query(WrappedRequest { wait_masterchain_seqno: None, request: Request::GetTime }).await?;
    println!("{:?}", result);
    Ok(())
}