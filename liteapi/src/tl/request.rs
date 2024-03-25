use derivative::Derivative;
use tl_proto::{TlRead, TlWrite};

use super::common::*;
use super::utils::*;

/// liteServer.query data:bytes = Object;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(
    boxed,
    id = "liteServer.query",
    scheme_inline = r##"liteServer.query data:bytes = Object;"##
)]
pub struct Query {
    #[tl(with = "struct_as_bytes")]
    pub wrapped_request: WrappedRequest,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct WrappedRequest {
    #[tl(read_with = "lossy_read")]
    pub wait_masterchain_seqno: Option<WaitMasterchainSeqno>,
    pub request: Request,
}

/// liteServer.query data:bytes = Object;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(
    boxed,
    id = "liteServer.waitMasterchainSeqno",
    scheme_inline = r##"liteServer.waitMasterchainSeqno seqno:int timeout_ms:int = Object;"##
)]
pub struct WaitMasterchainSeqno {
    pub seqno: i32,
    pub timeout_ms: i32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetMasterchainInfoExt {
    mode: u32
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetBlock {
    id: BlockIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetState {
    id: BlockIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetBlockHeader {
    id: BlockIdExt,
    mode: u32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct SendMessage {
    body: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetAccountState {
    id: BlockIdExt,
    account: AccountId,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct RunSmcMethod {
    mode: u32,
    id: BlockIdExt,
    account: AccountId,
    method_id: i64,
    params: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetShardInfo {
    id: BlockIdExt,
    workchain: i32,
    shard: u64,
    exact: bool,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetAllShardsInfo {
    id: BlockIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetOneTransaction {
    id: BlockIdExt,
    account: AccountId,
    lt: i64,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetTransactions {
    count: i32,
    account: AccountId,
    lt: i64,
    hash: Int256,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct LookupBlock {
    #[tl(flags)]
    mode: (),
    id: BlockId,
    #[tl(flags_bit = "mode.1")]
    lt: Option<i64>,
    #[tl(flags_bit = "mode.2")]
    utime: Option<i32>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ListBlockTransactions {
    id: BlockIdExt,
    #[tl(flags)]
    mode: (),
    count: i32,
    #[tl(flags_bit = "mode.7")]
    after: Option<TransactionId3>,
    #[tl(flags_bit = "mode.6")]
    reverse_order: Option<()>,
    #[tl(flags_bit = "mode.5")]
    want_proof: Option<()>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetBlockProof {
    #[tl(flags)]
    mode: (),
    known_block: BlockIdExt,
    #[tl(flags_bit = "mode.0")]
    target_block: Option<BlockIdExt>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetConfigAll {
    mode: i32,
    id: BlockIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetConfigParams {
    mode: i32,
    id: BlockIdExt,
    param_list: Vec<i32>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetValidatorStats {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
    limit: i32,
    #[tl(flags_bit = "mode.0")]
    start_after: Option<Int256>,
    #[tl(flags_bit = "mode.2")]
    modified_after: Option<i32>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed)]
pub enum Request {
    /// liteServer.getMasterchainInfo = liteServer.MasterchainInfo;
    #[tl(id = 0x89b5e62e)]
    GetMasterchainInfo,

    /// liteServer.getMasterchainInfoExt mode:# = liteServer.MasterchainInfoExt;
    #[tl(id = 0x70a671df)]
    GetMasterchainInfoExt(GetMasterchainInfoExt),

    /// liteServer.getTime = liteServer.CurrentTime;
    #[tl(id = 0x16ad5a34)]
    GetTime,

    /// liteServer.getVersion = liteServer.Version;
    #[tl(id = 0x232b940b)]
    GetVersion,

    /// liteServer.getBlock id:tonNode.blockIdExt = liteServer.BlockData;
    #[tl(id = 0x6377cf0d)]
    GetBlock(GetBlock),

    /// liteServer.getState id:tonNode.blockIdExt = liteServer.BlockState;
    #[tl(id = 0xba6e2eb6)]
    GetState(GetState),

    /// liteServer.getBlockHeader id:tonNode.blockIdExt mode:# = liteServer.BlockHeader;
    #[tl(id = 0x21ec069e)]
    GetBlockHeader(GetBlockHeader),

    /// liteServer.sendMessage body:bytes = liteServer.SendMsgStatus;
    #[tl(id = 0x690ad482)]
    SendMessage(SendMessage),

    /// liteServer.getAccountState id:tonNode.blockIdExt account:liteServer.accountId = liteServer.AccountState;
    #[tl(id = 0x6b890e25)]
    GetAccountState(GetAccountState),

    /// liteServer.runSmcMethod mode:# id:tonNode.blockIdExt account:liteServer.accountId method_id:long params:bytes = liteServer.RunMethodResult;
    #[tl(id = 0x5cc65dd2)]
    RunSmcMethod(RunSmcMethod),

    /// liteServer.getShardInfo id:tonNode.blockIdExt workchain:int shard:long exact:Bool = liteServer.ShardInfo;
    #[tl(id = 0x46a2f425)]
    GetShardInfo(GetShardInfo),

    /// liteServer.getAllShardsInfo id:tonNode.blockIdExt = liteServer.AllShardsInfo;
    #[tl(id = 0x74d3fd6b)]
    GetAllShardsInfo(GetAllShardsInfo),

    /// liteServer.getOneTransaction id:tonNode.blockIdExt account:liteServer.accountId lt:long = liteServer.TransactionInfo;
    #[tl(id = 0xd40f24ea)]
    GetOneTransaction(GetOneTransaction),

    /// liteServer.getTransactions count:# account:liteServer.accountId lt:long hash:int256 = liteServer.TransactionList;
    #[tl(id = 0x1c40e7a1)]
    GetTransactions(GetTransactions),

    /// liteServer.lookupBlock mode:# id:tonNode.blockId lt:mode.1?long utime:mode.2?int = liteServer.BlockHeader;
    #[tl(id = 0xfac8f71e)]
    LookupBlock(LookupBlock),

    /// liteServer.listBlockTransactions id:tonNode.blockIdExt mode:# count:# after:mode.7?liteServer.transactionId3 reverse_order:mode.6?true want_proof:mode.5?true = liteServer.BlockTransactions;
    #[tl(id = 0xadfcc7da)]
    ListBlockTransactions(ListBlockTransactions),

    /// liteServer.getBlockProof mode:# known_block:tonNode.blockIdExt target_block:mode.0?tonNode.blockIdExt = liteServer.PartialBlockProof;
    #[tl(id = 0x8aea9c44)]
    GetBlockProof(GetBlockProof),

    /// liteServer.getConfigAll mode:# id:tonNode.blockIdExt = liteServer.ConfigInfo;
    #[tl(id = 0x911b26b7)]
    GetConfigAll(GetConfigAll),

    /// liteServer.getConfigParams mode:# id:tonNode.blockIdExt param_list:(vector int) = liteServer.ConfigInfo;
    #[tl(id = 0x2a111c19)]
    GetConfigParams(GetConfigParams),

    /// liteServer.getValidatorStats#091a58bc mode:# id:tonNode.blockIdExt limit:int start_after:mode.0?int256 modified_after:mode.2?int = liteServer.ValidatorStats;
    #[tl(id = 0xe7253699)]
    GetValidatorStats(GetValidatorStats),
}