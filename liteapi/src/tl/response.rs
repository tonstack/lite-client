use derivative::Derivative;
use tl_proto::{TlRead, TlWrite};

use super::common::*;
use super::utils::*;

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct MasterchainInfo {
    pub last: BlockIdExt,
    pub state_root_hash: Int256,
    pub init: ZeroStateIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct MasterchainInfoExt {
    #[tl(flags)]
    pub mode: (),
    pub version: u32,
    pub capabilities: u64,
    pub last: BlockIdExt,
    pub last_utime: u32,
    pub now: u32,
    pub state_root_hash: Int256,
    pub init: ZeroStateIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct CurrentTime {
    pub now: u32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Version {
    pub mode: u32,
    pub version: u32,
    pub capabilities: u64,
    pub now: u32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockData {
    pub id: BlockIdExt,
    #[derivative(Debug(format_with="fmt_bytes"))]
    pub data: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockState {
    pub id: BlockIdExt,
    pub root_hash: Int256,
    pub file_hash: Int256,
    #[derivative(Debug(format_with="fmt_bytes"))]
    pub data: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockHeader {
    pub id: BlockIdExt,
    #[tl(flags)]
    pub mode: (),
    #[tl(flags_bit = "mode.0")]
    pub with_state_update: Option<()>,
    #[tl(flags_bit = "mode.1")]
    pub with_value_flow: Option<()>,
    #[tl(flags_bit = "mode.4")]
    pub with_extra: Option<()>,
    #[tl(flags_bit = "mode.5")]
    pub with_shard_hashes: Option<()>,
    #[tl(flags_bit = "mode.6")]
    pub with_prev_blk_signatures: Option<()>,
    #[derivative(Debug(format_with="fmt_bytes"))]
    pub header_proof: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct SendMsgStatus {
    pub status: u32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct AccountState {
    pub id: BlockIdExt,
    pub shardblk: BlockIdExt,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub shard_proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub state: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct RunMethodResult {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub shardblk: BlockIdExt,
    #[tl(flags_bit = "mode.0")]
    #[derivative(Debug(format_with="fmt_opt_bytes"))] 
    pub shard_proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.0")]
    #[derivative(Debug(format_with="fmt_opt_bytes"))] 
    pub proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.1")]
    #[derivative(Debug(format_with="fmt_opt_bytes"))] 
    pub state_proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.3")]
    #[derivative(Debug(format_with="fmt_opt_bytes"))] 
    pub init_c7: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.4")]
    #[derivative(Debug(format_with="fmt_opt_bytes"))] 
    pub lib_extras: Option<Vec<u8>>,
    pub exit_code: i32,
    #[tl(flags_bit = "mode.2")]
    #[derivative(Debug(format_with="fmt_opt_bytes"))] 
    pub result: Option<Vec<u8>>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ShardInfo {
    pub id: BlockIdExt,
    pub shardblk: BlockIdExt,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub shard_proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub shard_descr: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct AllShardsInfo {
    pub id: BlockIdExt,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub data: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct TransactionInfo {
    pub id: BlockIdExt,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub transaction: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct TransactionList {
    pub ids: Vec<BlockIdExt>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub transactions: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct TransactionMetadata {
    #[tl(flags)]
    mode: (),
    depth: u32,
    initiator: AccountId,
    initiator_lt: u64,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct TransactionId {
    #[tl(flags)]
    pub mode: (),
    #[tl(flags_bit = "mode.0")]
    pub account: Option<Int256>,
    #[tl(flags_bit = "mode.1")]
    pub lt: Option<u64>,
    #[tl(flags_bit = "mode.2")]
    pub hash: Option<Int256>,
    #[tl(flags_bit = "mode.8")]
    pub metadata: Option<TransactionMetadata>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockTransactions {
    pub id: BlockIdExt,
    pub req_count: u32,
    pub incomplete: bool,
    pub ids: Vec<TransactionId>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub proof: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct PartialBlockProof {
    pub complete: bool,
    pub from: BlockIdExt,
    pub to: BlockIdExt,
    pub steps: Vec<BlockLink>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ConfigInfo {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub state_proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub config_proof: Vec<u8>,
    #[tl(flags_bit = "mode.0")]
    pub with_state_root: Option<()>,
    #[tl(flags_bit = "mode.1")]
    pub with_libraries: Option<()>,
    #[tl(flags_bit = "mode.2")]
    pub with_state_extra_root: Option<()>,
    #[tl(flags_bit = "mode.3")]
    pub with_shard_hashes: Option<()>,
    #[tl(flags_bit = "mode.4")]
    pub with_validator_set: Option<()>,
    #[tl(flags_bit = "mode.5")]
    pub with_special_smc: Option<()>,
    #[tl(flags_bit = "mode.6")]
    pub with_accounts_root: Option<()>,
    #[tl(flags_bit = "mode.7")]
    pub with_prev_blocks: Option<()>,
    #[tl(flags_bit = "mode.8")]
    pub with_workchain_info: Option<()>,
    #[tl(flags_bit = "mode.9")]
    pub with_capabilities: Option<()>,
    #[tl(flags_bit = "mode.15")]
    pub extract_from_key_block: Option<()>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ValidatorStats {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub count: u32,
    pub complete: bool,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub state_proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub data_proof: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct LibraryResult {
    pub result: Vec<LibraryEntry>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct LibraryResultWithProof {
    pub id: BlockIdExt,
    #[tl(flags)]
    pub mode: (),
    pub result: Vec<LibraryEntry>,
    pub state_proof: Vec<u8>,
    pub data_proof: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Error {
    pub code: i32,
    #[derivative(Debug(format_with = "String::fmt"))]
    pub message: String,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed)]
pub enum Response {
    /// liteServer.masterchainInfo last:tonNode.blockIdExt state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfo;
    #[tl(id = 0x85832881)]
    MasterchainInfo(MasterchainInfo),

    /// liteServer.masterchainInfoExt mode:# version:int capabilities:long last:tonNode.blockIdExt last_utime:int now:int state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfoExt;
    #[tl(id = 0xa8cce0f5)]
    MasterchainInfoExt(MasterchainInfoExt),

    /// liteServer.currentTime now:int = liteServer.CurrentTime;
    #[tl(id = 0xe953000d)]
    CurrentTime(CurrentTime),

    /// liteServer.version mode:# version:int capabilities:long now:int = liteServer.Version;
    #[tl(id = 0x5a0491e5)]
    Version(Version),

    /// liteServer.blockData id:tonNode.blockIdExt data:bytes = liteServer.BlockData;
    #[tl(id = 0xa574ed6c)]
    BlockData(BlockData),

    /// liteServer.blockState id:tonNode.blockIdExt root_hash:int256 file_hash:int256 data:bytes = liteServer.BlockState;
    #[tl(id = 0xabaddc0c)]
    BlockState(BlockState),

    /// liteServer.blockHeader id:tonNode.blockIdExt mode:# header_proof:bytes = liteServer.BlockHeader;
    #[tl(id = 0x752d8219)]
    BlockHeader(BlockHeader),

    /// liteServer.sendMsgStatus status:int = liteServer.SendMsgStatus;
    #[tl(id = 0x3950e597)]
    SendMsgStatus(SendMsgStatus),

    /// liteServer.accountState id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes proof:bytes state:bytes = liteServer.AccountState;
    #[tl(id = 0x7079c751)]
    AccountState(AccountState),

    /// liteServer.runMethodResult mode:# id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:mode.0?bytes proof:mode.0?bytes state_proof:mode.1?bytes init_c7:mode.3?bytes lib_extras:mode.4?bytes exit_code:int result:mode.2?bytes = liteServer.RunMethodResult;
    #[tl(id = 0xa39a616b)]
    RunMethodResult(RunMethodResult),

    /// liteServer.shardInfo id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes shard_descr:bytes = liteServer.ShardInfo;
    #[tl(id = 0x9fe6cd84)]
    ShardInfo(ShardInfo),

    /// liteServer.allShardsInfo id:tonNode.blockIdExt proof:bytes data:bytes = liteServer.AllShardsInfo;
    #[tl(id = 0x098fe72d)]
    AllShardsInfo(AllShardsInfo),

    /// liteServer.transactionInfo id:tonNode.blockIdExt proof:bytes transaction:bytes = liteServer.TransactionInfo;
    #[tl(id = 0x0edeed47)]
    TransactionInfo(TransactionInfo),

    /// liteServer.transactionList ids:(vector tonNode.blockIdExt) transactions:bytes = liteServer.TransactionList;
    #[tl(id = 0x6f26c60b)]
    TransactionList(TransactionList),

    /// liteServer.transactionId mode:# account:mode.0?int256 lt:mode.1?long hash:mode.2?int256 metadata:mode.8?liteServer.transactionMetadata = liteServer.TransactionId;
    #[tl(id = 0xb12f65af)]
    TransactionId(TransactionId),

    /// liteServer.blockTransactions id:tonNode.blockIdExt req_count:# incomplete:Bool ids:(vector liteServer.transactionId) proof:bytes = liteServer.BlockTransactions;
    #[tl(id = 0xbd8cad2b)]
    BlockTransactions(BlockTransactions),

    /// liteServer.partialBlockProof complete:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt steps:(vector liteServer.BlockLink) = liteServer.PartialBlockProof;
    #[tl(id = 0x8ed0d2c1)]
    PartialBlockProof(PartialBlockProof),

    /// liteServer.configInfo mode:# id:tonNode.blockIdExt state_proof:bytes config_proof:bytes = liteServer.ConfigInfo;
    #[tl(id = 0xae7b272f)]
    ConfigInfo(ConfigInfo),

    /// liteServer.validatorStats mode:# id:tonNode.blockIdExt count:int complete:Bool state_proof:bytes data_proof:bytes = liteServer.ValidatorStats;
    #[tl(id = 0xb9f796d8)]
    ValidatorStats(ValidatorStats),

    /// liteServer.libraryResult result:(vector liteServer.libraryEntry) = liteServer.LibraryResult;
    #[tl(id = 0x117ab96b)]
    LibraryResult(LibraryResult),

    /// liteServer.libraryResult result:(vector liteServer.libraryEntry) = liteServer.LibraryResult;
    #[tl(id = 0x10a927bf)]
    LibraryResultWithProof(LibraryResultWithProof),

    /// liteServer.error code:int message:string = liteServer.Error;
    #[tl(id = 0xbba9e148)]
    Error(Error),
}