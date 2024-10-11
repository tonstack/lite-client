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
pub struct LiteQuery {
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
    pub seqno: u32,
    pub timeout_ms: u32,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetMasterchainInfoExt {
    pub mode: u32
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetBlock {
    pub id: BlockIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetState {
    pub id: BlockIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetBlockHeader {
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
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct SendMessage {
    pub body: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetAccountState {
    pub id: BlockIdExt,
    pub account: AccountId,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct RunSmcMethod {
    pub mode: u32,
    pub id: BlockIdExt,
    pub account: AccountId,
    pub method_id: u64,
    pub params: Vec<u8>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetShardInfo {
    pub id: BlockIdExt,
    pub workchain: i32,
    pub shard: u64,
    pub exact: bool,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetAllShardsInfo {
    pub id: BlockIdExt,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetOneTransaction {
    pub id: BlockIdExt,
    pub account: AccountId,
    pub lt: u64,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetTransactions {
    pub count: u32,
    pub account: AccountId,
    pub lt: u64,
    pub hash: Int256,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct LookupBlock {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockId,
    #[tl(flags_bit = "mode.0")]
    pub seqno: Option<()>,
    #[tl(flags_bit = "mode.1")]
    pub lt: Option<u64>,
    #[tl(flags_bit = "mode.2")]
    pub utime: Option<u32>,
    #[tl(flags_bit = "mode.4")]
    pub with_state_update: Option<()>,
    #[tl(flags_bit = "mode.5")]
    pub with_value_flow: Option<()>,
    #[tl(flags_bit = "mode.8")]
    pub with_extra: Option<()>,
    #[tl(flags_bit = "mode.9")]
    pub with_shard_hashes: Option<()>,
    #[tl(flags_bit = "mode.10")]
    pub with_prev_blk_signatures: Option<()>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct ListBlockTransactions {
    pub id: BlockIdExt,
    #[tl(flags)]
    pub mode: (),
    pub count: u32,
    #[tl(flags_bit = "mode.7")]
    pub after: Option<TransactionId3>,
    #[tl(flags_bit = "mode.6")]
    pub reverse_order: Option<()>,
    #[tl(flags_bit = "mode.5")]
    pub want_proof: Option<()>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetBlockProof {
    #[tl(flags)]
    pub mode: (),
    pub known_block: BlockIdExt,
    #[tl(flags_bit = "mode.0")]
    pub target_block: Option<BlockIdExt>,
    #[tl(flags_bit = "mode.1")]
    pub allow_weak_target: Option<()>,
    #[tl(flags_bit = "mode.12")]
    pub base_block_from_request: Option<()>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetConfigAll {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
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
pub struct GetConfigParams {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub param_list: Vec<i32>,
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
pub struct GetValidatorStats {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub limit: u32,
    #[tl(flags_bit = "mode.0")]
    pub start_after: Option<Int256>,
    #[tl(flags_bit = "mode.2")]
    pub modified_after: Option<u32>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetLibraries {
    pub library_list: Vec<Int256>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct GetLibrariesWithProof {
    pub id: BlockIdExt,
    #[tl(flags)]
    pub mode: (),
    pub library_list: Vec<Int256>,
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

    /// liteServer.getLibraries library_list:(vector int256) = liteServer.LibraryResult;
    #[tl(id = 0xd122b662)]
    GetLibraries(GetLibraries),

    /// liteServer.getLibrariesWithProof id:tonNode.blockIdExt mode:# library_list:(vector int256) = liteServer.LibraryResultWithProof;
    #[tl(id = 0xd97693bd)]
    GetLibrariesWithProof(GetLibrariesWithProof),
}
