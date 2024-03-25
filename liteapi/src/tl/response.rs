use derivative::Derivative;
use tl_proto::{TlRead, TlWrite};

use super::common::*;
use super::utils::*;

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct MasterchainInfo {
    last: BlockIdExt,
    state_root_hash: Int256,
    init: ZeroStateIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct MasterchainInfoExt {
    #[tl(flags)]
    mode: (),
    version: i32,
    capabilities: i64,
    last: BlockIdExt,
    last_utime: i32,
    now: i32,
    state_root_hash: Int256,
    init: ZeroStateIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct CurrentTime {
    now: i32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Version {
    mode: i32,
    version: i32,
    capabilities: i64,
    now: i32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockData {
    id: BlockIdExt,
    data: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockState {
    id: BlockIdExt,
    root_hash: Int256,
    file_hash: Int256,
    data: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockHeader {
    id: BlockIdExt,
    mode: i32,
    header_proof: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct SendMsgStatus {
    status: i32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct AccountState {
    id: BlockIdExt,
    shardblk: BlockIdExt,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    shard_proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    proof: Vec<u8>,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    state: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct RunMethodResult {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
    shardblk: BlockIdExt,
    #[tl(flags_bit = "mode.0")]
    shard_proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.0")]
    proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.1")]
    state_proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.3")]
    init_c7: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.4")]
    lib_extras: Option<Vec<u8>>,
    exit_code: i32,
    #[tl(flags_bit = "mode.2")]
    result: Option<Vec<u8>>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ShardInfo {
    id: BlockIdExt,
    shardblk: BlockIdExt,
    shard_proof: Vec<u8>,
    shard_descr: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct AllShardsInfo {
    id: BlockIdExt,
    proof: Vec<u8>,
    data: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct TransactionInfo {
    id: BlockIdExt,
    proof: Vec<u8>,
    transaction: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct TransactionList {
    ids: Vec<BlockIdExt>,
    transactions: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct TransactionId {
    #[tl(flags)]
    mode: (),
    #[tl(flags_bit = "mode.0")]
    account: Option<Int256>,
    #[tl(flags_bit = "mode.1")]
    lt: Option<i64>,
    #[tl(flags_bit = "mode.2")]
    hash: Option<Int256>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockTransactions {
    id: BlockIdExt,
    req_count: i32,
    incomplete: bool,
    ids: Vec<TransactionId>,
    proof: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct PartialBlockProof {
    complete: bool,
    from: BlockIdExt,
    to: BlockIdExt,
    steps: Vec<BlockLink>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ConfigInfo {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
    state_proof: Vec<u8>,
    config_proof: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ValidatorStats {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
    count: i32,
    complete: bool,
    state_proof: Vec<u8>,
    data_proof: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Error {
    code: i32,
    #[derivative(Debug(format_with = "String::fmt"))]
    message: String,
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

    /// liteServer.transactionId mode:# account:mode.0?int256 lt:mode.1?long hash:mode.2?int256 = liteServer.TransactionId;
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

    /// liteServer.error code:int message:string = liteServer.Error;
    #[tl(id = 0xbba9e148)]
    Error(Error),
}