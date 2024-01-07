mod arg_parsers;

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use adnl::AdnlPublicKey;
use rand::seq::SliceRandom;
use ton_liteapi::tl_types::{BlockIdExt, Int256, AccountId};
use ton_liteapi::LiteClient;
use pretty_hex::PrettyHex;
use ton_networkconfig::{ConfigGlobal, ConfigLiteServer, ConfigPublicKey};
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{stdin, Read};
use std::net::TcpStream;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};

use crate::arg_parsers::{parse_account_id, parse_block_id_ext};

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
    /// Get masterchain info
    GetMasterchainInfo,
    /// Get masterchain info with additional data
    GetMasterchainInfoExt {
        mode: i32,
    },
    /// Get server time
    GetTime,
    /// Shows server time, version and capabilities
    GetVersion,
    /// Downloads and dumps specified block
    #[clap(arg_required_else_help = true)]
    GetBlock {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
    },
    GetState {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
    },
    GetBlockHeader {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        mode: i32,
    },
    /// Send external message
    #[clap(arg_required_else_help = true, parse(from_os_str))]
    SendMessage {
        /// File to send
        file: PathBuf,
    },
    GetAccountState {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        #[clap(value_parser = parse_account_id)]
        account_id: AccountId,
    },
    RunSmcMethod {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        #[clap(value_parser = parse_account_id)]
        account_id: AccountId,
        method_id: i64,
        params: Vec<u8>,
    },
    GetShardInfo {
        seqno: u32,
        workchain: i32,
    },
    GetAllShardsInfo {
        seqno: u32,
    },
    GetOneTransaction {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        #[clap(value_parser = parse_account_id)]
        account_id: AccountId,
        lt: i64,
    },
    GetTransactions {
        count: i32,
        #[clap(value_parser = parse_account_id)]
        account_id: AccountId,
        lt: i64,
        hash: Int256,
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
    ListBlockTransactions {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        count: i32,
        #[clap(short, long, parse(from_flag))]
        reverse_order: bool,
        #[clap(short, long, parse(from_flag))]
        want_proof: bool,
        #[clap(requires = "after-lt", value_parser = parse_account_id, long)]
        after_account: Option<AccountId>,
        #[clap(requires = "after-account", long)]
        after_lt: Option<i64>,
    },
    GetBlockProof {
        #[clap(value_parser = parse_block_id_ext)]
        known_block: BlockIdExt,
        #[clap(value_parser = parse_block_id_ext)]
        target_block: Option<BlockIdExt>,
    },
    GetConfigAll {
        mode: i32,
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
    },
    GetConfigParams {
        mode: i32,
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        param_list: Vec<i32>,
    },
    GetValidatorStats {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        limit: i32,
        start_after: Option<Int256>,
        modified_after: Option<i32>,
    },
}

fn execute_command(client: &mut LiteClient<TcpStream>, command: &Commands) -> Result<()> {
    match command {
        Commands::GetMasterchainInfo => {
            let result = (*client).get_masterchain_info()?;
            println!("{:#?}\n", result);
            println!("Last masterchain BlockIdExt: {}", result.last);
        }
        Commands::GetMasterchainInfoExt { mode } => {
            let result = (*client).get_masterchain_info_ext(0)?;
            println!("{:#?}\n", result);
            println!("Last masterchain BlockIdExt: {}", result.last);
        }
        Commands::GetTime => {
            let result = (*client).get_time()?.now as u64;
            let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(result));
            println!("Current time: {} => {:?}", result, time);
        }
        Commands::GetVersion => {
            let result = (*client).get_version()?;
            println!("Current version: {:?}", result);
        }
        Commands::GetBlock { block_id_ext } => {
            let result = (*client).get_block(block_id_ext.clone())?;
            println!("BlockData: {:?}", result.data.hex_dump());
        }
        Commands::GetState { block_id_ext } => {
            let result = (*client).get_state(block_id_ext.clone())?;
        }
        Commands::GetBlockHeader { block_id_ext, mode } => {

        }
        Commands::SendMessage { file } => {
            let mut data = Vec::<u8>::new();
            if file.to_str().map(|f| f == "-").unwrap_or(false) {
                stdin().read_to_end(&mut data)?;
            } else {
                File::open(file)?.read_to_end(&mut data)?;
            }
            let result = client.send_message(data)?;
            println!("result = {:?}", result);
        }
        Commands::GetAccountState { block_id_ext, account_id } => {
            let result = (*client).get_account_state(block_id_ext.clone(), account_id.clone())?;
            println!("{:#?}", result);
        }
        Commands::RunSmcMethod { block_id_ext, account_id, method_id, params } => {

        }
        Commands::GetShardInfo { seqno, workchain } => {
            let block = (*client).lookup_block(
                ton_liteapi::tl_types::BlockId {
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
                ton_liteapi::tl_types::BlockId {
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
        Commands::GetOneTransaction { block_id_ext, account_id, lt } => {

        }
        Commands::GetTransactions { count, account_id, lt, hash } => {

        }
        Commands::LookupBlock {
            seqno,
            ltime,
            utime,
            workchain,
            shard,
        } => {
            let block = ton_liteapi::tl_types::BlockId {
                seqno: if let Some(seqno) = seqno { *seqno } else { 0 },
                shard: *shard,
                workchain: *workchain,
            };
            let res = (*client).lookup_block(block, *ltime, *utime).unwrap();
            println!("{:#?}\n", res);
            println!("BlockIdExt: {}", res.id);
        }
        Commands::ListBlockTransactions { block_id_ext, count, reverse_order, want_proof, after_account, after_lt } => {

        }
        Commands::GetBlockProof { known_block, target_block } => {
            let result = (*client).get_block_proof(known_block.clone(), None)?;
            println!("{:?}", &result);
        }
        Commands::GetConfigAll { mode, block_id_ext } => {

        }
        Commands::GetConfigParams { mode, block_id_ext, param_list } => {

        }
        Commands::GetValidatorStats { block_id_ext, limit, start_after, modified_after } => {
            
        }
    };
    Ok(())
}

fn download_config(testnet: bool) -> Result<String> {
    let url = if testnet {
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
    Ok(response.into_string()?)
}

fn connect_ls(ls: &ConfigLiteServer) -> Result<LiteClient<TcpStream>> {
    let transport = TcpStream::connect_timeout(&ls.socket_addr().into(), Duration::from_secs(2))?;
    Ok(LiteClient::connect::<ConfigPublicKey>(transport, ls.id.clone())?)
}

fn connect_any(config: &ConfigGlobal) -> Result<LiteClient<TcpStream>> {
    for ls in &config.liteservers {
        log::info!("Connecting to {} (id {})", ls.socket_addr(), base64::encode(ls.id.to_bytes()));
        match connect_ls(ls) {
            Ok(x) => return Ok(x),
            Err(e) => {
                log::warn!("Connection error: {}", e);
            },
        }
    }
    return Err("No liteservers available".into())
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    let config_json = if let Some(config) = args.config {
        read_to_string(config)?
    } else {
        download_config(args.testnet)?
    };
    let mut config: ConfigGlobal = ConfigGlobal::from_str(&config_json)?;
    config.liteservers.shuffle(&mut rand::thread_rng());
    let mut client = connect_any(&config)?;
    if let Err(e) = execute_command(&mut client, &args.command) {
        println!("[ERROR] {}", e);
    }
    Ok(())
}
