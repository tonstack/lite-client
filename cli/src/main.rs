mod arg_parsers;

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use rand::seq::SliceRandom as _;
use ton_liteapi::tl::common::{AccountId, BlockId, BlockIdExt, Int256, TransactionId3};
use ton_liteapi::client::LiteClient;
use pretty_hex::PrettyHex;
use ton_networkconfig::ConfigGlobal;
use std::error::Error;
use std::fs::{read_to_string, File};
use std::io::{stdin, Read};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Duration, UNIX_EPOCH};

use crate::arg_parsers::{parse_account_id, parse_block_id_ext, parse_key};

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
    /// Liteserver address (IP:PORT)
    #[clap(long, group = "config-group")]
    address: Option<SocketAddr>,
    /// Liteserver public key (hex-encoded)
    #[clap(long, value_parser = parse_key, requires = "address")]
    public_key: Option<[u8; 32]>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get masterchain info
    GetMasterchainInfo,
    /// Get masterchain info with additional data
    GetMasterchainInfoExt {
        mode: u32,
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
        #[clap(long)]
        with_state_update: bool,
        #[clap(long)]
        with_value_flow: bool,
        #[clap(long)]
        with_extra: bool,
        #[clap(long)]
        with_shard_hashes: bool,
        #[clap(long)]
        with_prev_blk_signatures: bool,
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
        method_id: u64,
        params: Vec<u8>,
    },
    GetShardInfo {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        workchain: i32,
        shard: u64,
        exact: bool,
    },
    GetAllShardsInfo {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
    },
    GetOneTransaction {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        #[clap(value_parser = parse_account_id)]
        account_id: AccountId,
        lt: u64,
    },
    GetTransactions {
        count: u32,
        #[clap(value_parser = parse_account_id)]
        account_id: AccountId,
        lt: u64,
        hash: Int256,
    },
    LookupBlock {
        workchain: i32,
        shard: u64,
        #[clap(short, long, group = "lookup-variant")]
        seqno: Option<u32>,
        #[clap(short, long, group = "lookup-variant")]
        lt: Option<u64>,
        #[clap(short, long, group = "lookup-variant")]
        utime: Option<u32>,
        #[clap(long)]
        with_state_update: bool,
        #[clap(long)]
        with_value_flow: bool,
        #[clap(long)]
        with_extra: bool,
        #[clap(long)]
        with_shard_hashes: bool,
        #[clap(long)]
        with_prev_blk_signatures: bool,
    },
    ListBlockTransactions {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        count: u32,
        #[clap(short, long)]
        reverse_order: bool,
        #[clap(short, long)]
        want_proof: bool,
        #[clap(requires = "after-lt", value_parser = parse_account_id, long)]
        after_account: Option<AccountId>,
        #[clap(requires = "after-account", long)]
        after_lt: Option<u64>,
    },
    GetBlockProof {
        #[clap(value_parser = parse_block_id_ext)]
        known_block: BlockIdExt,
        #[clap(value_parser = parse_block_id_ext)]
        target_block: Option<BlockIdExt>,
        #[clap(long)]
        allow_weak_target: bool,
        #[clap(long)]
        base_block_from_request: bool,
    },
    GetConfigAll {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        #[clap(long)]
        with_state_root: bool,
        #[clap(long)]
        with_libraries: bool,
        #[clap(long)]
        with_state_extra_root: bool,
        #[clap(long)]
        with_shard_hashes: bool,
        #[clap(long)]
        with_validator_set: bool,
        #[clap(long)]
        with_special_smc: bool,
        #[clap(long)]
        with_accounts_root: bool,
        #[clap(long)]
        with_prev_blocks: bool,
        #[clap(long)]
        with_workchain_info: bool,
        #[clap(long)]
        with_capabilities: bool,
        #[clap(long)]
        extract_from_key_block: bool,
    },
    GetConfigParams {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        param_list: Vec<i32>,
        #[clap(long)]
        with_state_root: bool,
        #[clap(long)]
        with_libraries: bool,
        #[clap(long)]
        with_state_extra_root: bool,
        #[clap(long)]
        with_shard_hashes: bool,
        #[clap(long)]
        with_validator_set: bool,
        #[clap(long)]
        with_special_smc: bool,
        #[clap(long)]
        with_accounts_root: bool,
        #[clap(long)]
        with_prev_blocks: bool,
        #[clap(long)]
        with_workchain_info: bool,
        #[clap(long)]
        with_capabilities: bool,
        #[clap(long)]
        extract_from_key_block: bool,
    },
    GetValidatorStats {
        #[clap(value_parser = parse_block_id_ext)]
        block_id_ext: BlockIdExt,
        limit: u32,
        start_after: Option<Int256>,
        modified_after: Option<u32>,
    },
    GetLibraries {
        library_list: Vec<Int256>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();
    
    let client = if let (Some(address), Some(public_key)) = (&args.address, &args.public_key) {
        LiteClient::connect(address, public_key).await?
    } else {
        let config_json = if let Some(config) = args.config {
            read_to_string(config)?
        } else {
            download_config(args.testnet).await?
        };
        let config: ConfigGlobal = ConfigGlobal::from_str(&config_json)?;
        let ls = config.liteservers.choose(&mut rand::thread_rng()).unwrap();
        let public_key: [u8; 32] = ls.id.clone().into();
        LiteClient::connect(ls.socket_addr(), public_key).await?
    };

    let mut client = client;

    if let Err(e) = execute_command(&mut client, &args.command).await {
        println!("[ERROR] {:?}", e);
    }
    Ok(())
}

async fn execute_command(client: &mut LiteClient, command: &Commands) -> Result<()> {
    match command {
        Commands::GetMasterchainInfo => {
            let result = client.get_masterchain_info().await?;
            println!("{:#?}\n", result);
            println!("Last masterchain BlockIdExt: {}", result.last);
        }
        Commands::GetMasterchainInfoExt { mode } => {
            let result = client.get_masterchain_info_ext(*mode).await?;
            println!("{:#?}\n", result);
            println!("Last masterchain BlockIdExt: {}", result.last);
        }
        Commands::GetTime => {
            let result = client.get_time().await?;
            let time = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(result as u64));
            println!("Current time: {} => {:?}", result, time);
        }
        Commands::GetVersion => {
            let result = client.get_version().await?;
            println!("Current version: {:?}", result);
        }
        Commands::GetBlock { block_id_ext } => {
            let result = client.get_block(block_id_ext.clone()).await?;
            println!("BlockData: {:?}", result.hex_dump());
        }
        Commands::GetState { block_id_ext } => {
            let result = client.get_state(block_id_ext.clone()).await?;
            println!("{:#?}", result);
        }
        Commands::GetBlockHeader { block_id_ext, with_state_update, with_value_flow, with_extra, with_shard_hashes, with_prev_blk_signatures } => {
            let result = client.get_block_header(
                block_id_ext.clone(),
                *with_state_update,
                *with_value_flow,
                *with_extra,
                *with_shard_hashes,
                *with_prev_blk_signatures,
            ).await?;
            println!("Block Header: {:?}", result.hex_dump());
        }
        Commands::SendMessage { file } => {
            let mut data = Vec::<u8>::new();
            if file.to_str().map(|f| f == "-").unwrap_or(false) {
                stdin().read_to_end(&mut data)?;
            } else {
                File::open(file)?.read_to_end(&mut data)?;
            }
            let result = client.send_message(data).await?;
            println!("result = {:?}", result);
        }
        Commands::GetAccountState { block_id_ext, account_id } => {
            let result = client.get_account_state(block_id_ext.clone(), account_id.clone()).await?;
            println!("{:#?}", result);
        }
        Commands::RunSmcMethod { block_id_ext, account_id, method_id, params } => {
            let result = client.run_smc_method(0, block_id_ext.clone(), account_id.clone(), *method_id, params.clone()).await?;
            println!("{:#?}", result);
        }
        Commands::GetShardInfo { block_id_ext, workchain, shard, exact } => {
            let result = client.get_shard_info(block_id_ext.clone(), *workchain, *shard, *exact).await?;
            println!("{:#?}", result);
        }
        Commands::GetAllShardsInfo { block_id_ext } => {
            let result = client.get_all_shards_info(block_id_ext.clone()).await?;
            println!("{:#?}", result);
        }
        Commands::GetOneTransaction { block_id_ext, account_id, lt } => {
            let result = client.get_one_transaction(block_id_ext.clone(), account_id.clone(), *lt).await?;
            println!("{:#?}", result);
        }
        Commands::GetTransactions { count, account_id, lt, hash } => {
            let result = client.get_transactions(*count, account_id.clone(), *lt, hash.clone()).await?;
            println!("{:#?}", result);
        }
        Commands::LookupBlock { workchain, shard, seqno, lt, utime, with_state_update, with_value_flow, with_extra, with_shard_hashes, with_prev_blk_signatures } => {
            let result = client.lookup_block(
                (),
                BlockId { workchain: *workchain, shard: *shard, seqno: seqno.unwrap_or(0) },
                seqno.map(|_| ()),
                *lt,
                *utime,
                *with_state_update,
                *with_value_flow,
                *with_extra,
                *with_shard_hashes,
                *with_prev_blk_signatures,
            ).await?;
            println!("{:#?}", result);
        }
        Commands::ListBlockTransactions { block_id_ext, count, reverse_order, want_proof, after_account, after_lt } => {
            let after = after_account.as_ref().and_then(|account| after_lt.map(|lt| TransactionId3 {
                account: account.id.clone(),
                lt,
            }));
            let result = client.list_block_transactions(
                block_id_ext.clone(),
                *count,
                after,
                *reverse_order,
                *want_proof,
            ).await?;
            println!("{:#?}", result);
        }
        Commands::GetBlockProof { known_block, target_block, allow_weak_target, base_block_from_request } => {
            let result = client.get_block_proof(
                known_block.clone(),
                target_block.clone(),
                *allow_weak_target,
                *base_block_from_request,
            ).await?;
            println!("{:#?}", result);
        }
        Commands::GetConfigAll { block_id_ext, with_state_root, with_libraries, with_state_extra_root, with_shard_hashes, with_validator_set, with_special_smc, with_accounts_root, with_prev_blocks, with_workchain_info, with_capabilities, extract_from_key_block } => {
            let result = client.get_config_all(
                block_id_ext.clone(),
                *with_state_root,
                *with_libraries,
                *with_state_extra_root,
                *with_shard_hashes,
                *with_validator_set,
                *with_special_smc,
                *with_accounts_root,
                *with_prev_blocks,
                *with_workchain_info,
                *with_capabilities,
                *extract_from_key_block,
            ).await?;
            println!("{:#?}", result);
        }
        Commands::GetConfigParams { block_id_ext, param_list, with_state_root, with_libraries, with_state_extra_root, with_shard_hashes, with_validator_set, with_special_smc, with_accounts_root, with_prev_blocks, with_workchain_info, with_capabilities, extract_from_key_block } => {
            let result = client.get_config_params(
                block_id_ext.clone(),
                param_list.clone(),
                *with_state_root,
                *with_libraries,
                *with_state_extra_root,
                *with_shard_hashes,
                *with_validator_set,
                *with_special_smc,
                *with_accounts_root,
                *with_prev_blocks,
                *with_workchain_info,
                *with_capabilities,
                *extract_from_key_block,
            ).await?;
            println!("{:#?}", result);
        }
        Commands::GetValidatorStats { block_id_ext, limit, start_after, modified_after } => {
            let result = client.get_validator_stats(
                block_id_ext.clone(),
                *limit,
                start_after.clone(),
                *modified_after,
            ).await?;
            println!("{:#?}", result);
        }
        Commands::GetLibraries { library_list } => {
            let result = client.get_libraries(library_list.clone()).await?;
            println!("{:#?}", result);
        }
    };
    Ok(())
}

async fn download_config(testnet: bool) -> Result<String> {
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
