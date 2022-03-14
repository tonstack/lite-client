use std::error::Error;
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use liteclient::LiteClient;
use clap::{Parser, Subcommand};
use std::io::Read;
use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// LiteClient config
    #[clap(short, long, parse(from_os_str), value_name = "FILE", default_value = "./config.json")]
    config: PathBuf,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Send external message
    #[clap(arg_required_else_help = true, parse(from_os_str))]
    Send {
        /// File to send
        file: PathBuf,
    },
    /// Get time from liteserver
    GetTime,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let config = read_to_string(args.config)?;
    let mut client = LiteClient::connect(&config).await?;
    match &args.command {
        Commands::GetTime => {
            let result = client.get_time().await?;
            println!("Current time: {} => {:?}", result.now(), UNIX_EPOCH + Duration::from_secs(*result.now() as u64));
        }
        Commands::Send { file } => {
            let mut data = Vec::new();
            File::open(file)?.read_to_end(&mut data)?;
            let result = client.send_external_message(data).await?;
            println!("result = {:?}", result);
        }
    };

    Ok(())
}