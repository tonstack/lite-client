pub mod tl_types;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use liteclient::tl_types::{BlockIdExt, Int256};
use liteclient::LiteClient;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use pretty_hex::PrettyHex;
use regex::Regex;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Local network config from file
    #[clap(
        short,
        long,
        parse(from_os_str),
        value_name = "FILE",
        group = "config-group"
    )]
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
    /// Get server time
    GetTime,
    /// Shows server time, version and capabilities
    GetVersion,
    /// Get masterchain info
    GetMasterchainInfo,
    /// Get masterchain info with additional data
    GetMasterchainInfoExt,
    /// Downloads and dumps specified block
    #[clap(arg_required_else_help = true)]
    GetBlock {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
    },
    GetAccountState {
        base64_address: String,
    },
    LookupBlock {
        workchain: i32,
        shard: u64,
        #[clap(short, long, group = "lookup-variant")]
        seqno: Option<u32>,
        #[clap(short, long, group = "lookup-variant")]
        ltime: Option<i64>,
        #[clap(short, long, group = "lookup-variant")]
        utime: Option<i32>,
    },
    GetState {
        file_name: String,
    },
    GetShardInfo {
        seqno: u32,
        workchain: i32,
    },
    GetAllShardsInfo {
        seqno: u32,
    },
    GetBlockProof {
        seqno: u32,
    },
}

fn parse_block_id_ext(s: &str) -> std::result::Result<BlockIdExt, String> {
    let re = Regex::new(r"\(([-]?\d+),([a-fA-F0-9]+),(\d+)\):([^:]+):(.+)").unwrap();

    if let Some(capture) = re.captures(s) {
        let workchain = capture[1].parse::<i32>().map_err(|e| format!("Can't parse workchain '{}': {:?}", &capture[1], e))?;
        let shard = u64::from_str_radix(&capture[2], 16).map_err(|e| format!("Can't parse shard '{}': {:?}", &capture[2], e))?;
        let seqno = capture[3].parse::<u32>().map_err(|e| format!("Can't parse seqno '{}': {:?}", &capture[3], e))?;
        let root_hash = Int256::from_str(&capture[4]).map_err(|e| format!("Can't parse root_hash '{}': {:?}", &capture[4], e))?;
        let file_hash = Int256::from_str(&capture[5]).map_err(|e| format!("Can't parse root_hash '{}': {:?}", &capture[5], e))?;
        Ok(BlockIdExt { workchain, shard, seqno, root_hash, file_hash })
    } else {
        Err("Wrong format, must be (workchain,shard_hex,seqno)".into())
    }
}

fn execute_command(client: &mut LiteClient, command: &Commands) -> Result<()> {
    match command {
        Commands::GetTime => {
            let result = (*client).get_time()?.now as u64;
            let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(result));
            println!("Current time: {} => {:?}", result, time);
        }
        Commands::GetVersion => {
            let result = (*client).get_version()?;
            println!("Current version: {:?}", result);
        }
        Commands::GetMasterchainInfo => {
            let result = (*client).get_masterchain_info()?;
            println!("{:#?}\n", result);
            println!("Last masterchain BlockIdExt: {}", result.last);
        }
        Commands::GetMasterchainInfoExt => {
            let result = (*client).get_masterchain_info_ext(0)?;
            println!("{:#?}\n", result);
            println!("Last masterchain BlockIdExt: {}", result.last);
        }
        Commands::GetBlock { block_id_ext } => {
            let result = (*client).get_block(block_id_ext.clone())?;
            println!("BlockData: {:?}", result.data.hex_dump());
        }
        Commands::Send { file } => {
            let mut data = Vec::<u8>::new();
            if file.to_str().map(|f| f == "-").unwrap_or(false) {
                stdin().read_to_end(&mut data)?;
            } else {
                File::open(file)?.read_to_end(&mut data)?;
            }
            let result = client.send_message(data)?;
            println!("result = {:?}", result);
        }
        Commands::GetAccountState { base64_address } => {
            let info = (*client).get_masterchain_info_ext(0)?;
            let acc = liteclient::tl_types::AccountId::from_friendly(base64_address)?;
            let result = (*client).get_account_state(info.last, acc)?;
            println!("{:#?}", result);
        }
        Commands::LookupBlock {
            seqno,
            ltime,
            utime,
            workchain,
            shard,
        } => {
            let block = liteclient::tl_types::BlockId {
                seqno: if let Some(seqno) = seqno { *seqno } else { 0 },
                shard: *shard,
                workchain: *workchain,
            };
            let res = (*client).lookup_block(block, *ltime, *utime).unwrap();
            println!("{:#?}\n", res);
            println!("BlockIdExt: {}", res.id);
        }
        Commands::GetState { file_name } => {
            let workchain: i32 = -1;
            let shard: u64 = 9223372036854775808;
            let seqno: u32 = 999;
            let root_hash = liteclient::tl_types::Int256::from_base64(
                "46ZSUC+ehXaSunL740QURc7T6+o8CqykoT3Pg5Wbfak=",
            )
            .unwrap();
            let file_hash = liteclient::tl_types::Int256::from_base64(
                "Q8l3/cBazINazII5mOFSGg6/tuqiRKmdA3+Fjlrp/e4=",
            )
            .unwrap();
            let result = (*client).get_state(liteclient::tl_types::BlockIdExt {
                workchain,
                shard,
                seqno,
                root_hash,
                file_hash,
            })?;
            let mut file = File::create(file_name)?;
            file.write_all(&result.id.workchain.to_le_bytes())?;
            file.write_all(&result.id.seqno.to_le_bytes())?;
            file.write_all(&result.id.shard.to_le_bytes())?;
            file.write_all(&result.id.root_hash.0)?;
            file.write_all(&result.id.file_hash.0)?;
            file.write_all(&result.data)?;
        }
        Commands::GetShardInfo { seqno, workchain } => {
            let block = (*client).lookup_block(
                liteclient::tl_types::BlockId {
                    workchain: -1,
                    shard: 9223372036854775808,
                    seqno: *seqno,
                },
                None,
                None,
            )?;
            let result =
                (*client).get_shard_info(block.id, *workchain, 9223372036854775808, true)?;
            println!("{:?}", &result);
        }
        Commands::GetAllShardsInfo { seqno } => {
            let block = (*client).lookup_block(
                liteclient::tl_types::BlockId {
                    workchain: -1,
                    shard: 9223372036854775808,
                    seqno: *seqno,
                },
                None,
                None,
            )?;
            let result = (*client).get_all_shards_info(block.id)?;
            println!("{:?}", &result);
        }
        Commands::GetBlockProof { seqno } => {
            let block = (*client).lookup_block(
                liteclient::tl_types::BlockId {
                    workchain: -1,
                    shard: 9223372036854775808,
                    seqno: *seqno,
                },
                None,
                None,
            )?;
            let result = (*client).get_block_proof(block.id, None)?;
            println!("{:?}", &result);
        }
    };
    Ok(())
}

fn main() -> Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("logfile")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;
    let args = Args::parse();
    let config = if let Some(config) = args.config {
        read_to_string(config)?
    } else {
        let url = if args.testnet {
            "https://ton.org/testnet-global.config.json"
        } else {
            "https://ton.org/global.config.json"
        };
        let response = ureq::get(url).call()
            .map_err(|e| format!("Error occurred while fetching config from {}: {:?}. Use --config if you have local config.", url, e))?;
        if response.status() != 200 {
            return Err(format!(
                "Url {} responded with error code {}. Use --config if you have local config.",
                url,
                response.status()
            )
            .into());
        }
        response.into_string()?
    };
    let mut client = LiteClient::connect(&config)?;
    if let Err(e) = execute_command(&mut client, &args.command) {
        println!("[ERROR] {}", e);
    }
    Ok(())
}
