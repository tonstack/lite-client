use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use liteclient::LiteClient;
use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// LiteClient config
    #[clap(short, long, parse(from_os_str), value_name = "FILE", default_value = "./config.json")]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let config = read_to_string(args.config)?;
    let mut client = LiteClient::connect(&config).await?;
    let result = client.get_time().await?;
    println!("result = {:?}", result);
    Ok(())
}