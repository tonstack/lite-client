use ton_liteapi::client::LiteClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let server_public = hex::decode("9f69357376ad875d1543faea6f0bb9fbcd283521b743b1c0d2d432587fe9dbae")?;
    let server_address = ("127.0.0.1", 8080);
    let mut liteclient = LiteClient::connect(server_address, server_public).await?;
    let result = liteclient.get_time().await?;
    println!("{:?}", result);
    let result = liteclient.get_time().await?;
    println!("{:?}", result);
    let result = liteclient.get_time().await?;
    println!("{:?}", result);
    Ok(())
}