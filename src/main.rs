use std::error::Error;
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use liteclient::LiteClient;
use clap::{Parser, Subcommand};
use std::io::{Read, stdin};
use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Local network config from file
    #[clap(short, long, parse(from_os_str), value_name = "FILE", group = "config-group")]
    config: Option<PathBuf>,
    /// Use testnet config, if not provided use mainnet config
    #[clap(short, long, parse(from_flag), group = "config-group")]
    testnet: bool,
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

fn execute_command(client: &mut LiteClient, command: &Commands) -> Result<()> {
    match command {
        Commands::GetTime => {
            let result = *client.get_time()?.now() as u64;
            let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(result));
            println!("Current time: {} => {:?}", result, time);
        }
        Commands::Send { file } => {
            let mut data = Vec::new();
            if file.to_str().map(|f| f == "-").unwrap_or(false) {
                stdin().read_to_end(&mut data)?;
            } else {
                File::open(file)?.read_to_end(&mut data)?;
            }
            let result = client.send_external_message(data)?;
            println!("result = {:?}", result);
        }
    };
    Ok(())
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let config = if let Some(config) = args.config {
        read_to_string(config)?
    } else {
        let url = if args.testnet {
            "https://newton-blockchain.github.io/testnet-global.config.json"
        } else {
            "https://newton-blockchain.github.io/global.config.json"
        };
        let response = ureq::get(url).call()
            .map_err(|e| format!("Error occurred while fetching config from {}: {:?}. Use --config if you have local config.", url, e))?;
        if response.status() != 200 {
            return Err(format!("Url {} responded with error code {}. Use --config if you have local config.", url, response.status()).into());
        }
        response.into_string()?
    };
    let mut client = LiteClient::connect(&config)?;
    if let Err(e) = execute_command(&mut client, &args.command) {
        println!("[ERROR] {}", e);
    }
    Ok(())
}