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
    pub seqno: i32,
    pub timeout_ms: i32,
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
    GetMasterchainInfoExt {
        mode: u32,
    },

    /// liteServer.getTime = liteServer.CurrentTime;
    #[tl(id = 0x16ad5a34)]
    GetTime,

    /// liteServer.getVersion = liteServer.Version;
    #[tl(id = 0x232b940b)]
    GetVersion,

    /// liteServer.getBlock id:tonNode.blockIdExt = liteServer.BlockData;
    #[tl(id = 0x6377cf0d)]
    GetBlock {
        id: BlockIdExt,
    },

    /// liteServer.getState id:tonNode.blockIdExt = liteServer.BlockState;
    #[tl(id = 0xba6e2eb6)]
    GetState {
        id: BlockIdExt,
    },

    /// liteServer.getBlockHeader id:tonNode.blockIdExt mode:# = liteServer.BlockHeader;
    #[tl(id = 0x21ec069e)]
    GetBlockHeader {
        id: BlockIdExt,
        mode: u32,
    },

    /// liteServer.sendMessage body:bytes = liteServer.SendMsgStatus;
    #[tl(id = 0x690ad482)]
    SendMessage {
        body: Vec<u8>,
    },

    /// liteServer.getAccountState id:tonNode.blockIdExt account:liteServer.accountId = liteServer.AccountState;
    #[tl(id = 0x6b890e25)]
    GetAccountState {
        id: BlockIdExt,
        account: AccountId,
    },

    /// liteServer.runSmcMethod mode:# id:tonNode.blockIdExt account:liteServer.accountId method_id:long params:bytes = liteServer.RunMethodResult;
    #[tl(id = 0x5cc65dd2)]
    RunSmcMethod {
        mode: u32,
        id: BlockIdExt,
        account: AccountId,
        method_id: i64,
        params: Vec<u8>,
    },

    /// liteServer.getShardInfo id:tonNode.blockIdExt workchain:int shard:long exact:Bool = liteServer.ShardInfo;
    #[tl(id = 0x46a2f425)]
    GetShardInfo {
        id: BlockIdExt,
        workchain: i32,
        shard: u64,
        exact: bool,
    },

    /// liteServer.getAllShardsInfo id:tonNode.blockIdExt = liteServer.AllShardsInfo;
    #[tl(id = 0x74d3fd6b)]
    GetAllShardsInfo {
        id: BlockIdExt,
    },

    /// liteServer.getOneTransaction id:tonNode.blockIdExt account:liteServer.accountId lt:long = liteServer.TransactionInfo;
    #[tl(id = 0xd40f24ea)]
    GetOneTransaction {
        id: BlockIdExt,
        account: AccountId,
        lt: i64,
    },

    /// liteServer.getTransactions count:# account:liteServer.accountId lt:long hash:int256 = liteServer.TransactionList;
    #[tl(id = 0x1c40e7a1)]
    GetTransactions {
        count: i32,
        account: AccountId,
        lt: i64,
        hash: Int256,
    },

    /// liteServer.lookupBlock mode:# id:tonNode.blockId lt:mode.1?long utime:mode.2?int = liteServer.BlockHeader;
    #[tl(id = 0xfac8f71e)]
    LookupBlock {
        #[tl(flags)]
        mode: (),
        id: BlockId,
        //#[tl(skip, flags_bit = "mode.0")]
        //trash: Option<u8>,
        #[tl(flags_bit = "mode.1")]
        lt: Option<i64>,
        #[tl(flags_bit = "mode.2")]
        utime: Option<i32>,
    },

    /// liteServer.listBlockTransactions id:tonNode.blockIdExt mode:# count:# after:mode.7?liteServer.transactionId3 reverse_order:mode.6?true want_proof:mode.5?true = liteServer.BlockTransactions;
    #[tl(id = 0xadfcc7da)]
    ListBlockTransactions {
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
    },

    /// liteServer.getBlockProof mode:# known_block:tonNode.blockIdExt target_block:mode.0?tonNode.blockIdExt = liteServer.PartialBlockProof;
    #[tl(id = 0x8aea9c44)]
    GetBlockProof {
        #[tl(flags)]
        mode: (),
        known_block: BlockIdExt,
        #[tl(flags_bit = "mode.0")]
        target_block: Option<BlockIdExt>,
    },

    /// liteServer.getConfigAll mode:# id:tonNode.blockIdExt = liteServer.ConfigInfo;
    #[tl(id = 0x911b26b7)]
    GetConfigAll {
        mode: i32,
        id: BlockIdExt,
    },

    /// liteServer.getConfigParams mode:# id:tonNode.blockIdExt param_list:(vector int) = liteServer.ConfigInfo;
    #[tl(id = 0x2a111c19)]
    GetConfigParams {
        mode: i32,
        id: BlockIdExt,
        param_list: Vec<i32>,
    },

    /// liteServer.getValidatorStats#091a58bc mode:# id:tonNode.blockIdExt limit:int start_after:mode.0?int256 modified_after:mode.2?int = liteServer.ValidatorStats;
    #[tl(id = 0xe7253699)]
    GetValidatorStats {
        #[tl(flags)]
        mode: (),
        id: BlockIdExt,
        limit: i32,
        #[tl(flags_bit = "mode.0")]
        start_after: Option<Int256>,
        #[tl(flags_bit = "mode.2")]
        modified_after: Option<i32>,
    },
}