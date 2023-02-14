use std::error::Error;
use std::fs::{read_to_string, File};
use std::path::PathBuf;
use std::str::FromStr;
use liteclient::LiteClient;
use clap::{Parser, Subcommand};
use ton_block::BlockIdExt;
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
    /// Get version from liteserver
    GetVersion,
    /// Get masterchainInfo
    GetMasterchainInfo,
    #[clap(arg_required_else_help = true)]
    GetBlock {
        id: String,
    },
}

fn execute_command(client: &mut LiteClient, command: &Commands) -> Result<()> {
    match command {
        Commands::GetTime => {
            let result = (*client).get_time()?.now as u64;
            log::debug!("time: {}", result);
            let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(result));
            println!("Current time: {} => {:?}", result, time);
        }
        Commands::GetVersion => {
            let result = (*client).get_version()?;
            println!("Current version: {:?}", result);
        }
        Commands::GetMasterchainInfo => {
            let result = (*client).get_masterchain_info()?;
            println!("Last Block: {:?}", result);
        }
        Commands::Send { file } => todo!(),
        Commands::GetBlock { id } => todo!(),
        // Commands::Send { file } => {
        //     let mut data = Vec::new();
        //     if file.to_str().map(|f| f == "-").unwrap_or(false) {
        //         stdin().read_to_end(&mut data)?;
        //     } else {
        //         File::open(file)?.read_to_end(&mut data)?;
        //     }
        //     let result = client.send_message(data)?;
        //     println!("result = {:?}", result);
        // }

        // // cargo run get-block "(-1:8000000000000000, 25123484, rh 7f43835181544d3721196153f912226625568035627bdc5df827c983a4965cae, fh 36d45897be235ddc69abca3d35007fcdd15a8fbff41eb91efc64307a9a2cb0c7)"
        // Commands::GetBlock {id} => {
        //     let result = client.get_block(BlockIdExt::from_str(id.as_str())?)?;
        //     println!("BlockData: {:?}", result);
        // }
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
            "https://ton-blockchain.github.io/testnet-global.config.json"
        } else {
            "https://ton-blockchain.github.io/global.config.json"
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