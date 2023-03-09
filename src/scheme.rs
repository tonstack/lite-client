use tl_proto::{TlRead, TlWrite};
use std::str::FromStr;
use hex::{FromHex, FromHexError};
use derivative::Derivative;
use std::string::ToString;

/// true = True;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "true", scheme = "lite.tl")]
pub struct True;

/// string ? = String;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct String(Vec<u8>);

/// int128 4*[ int ] = Int128;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Int128(pub [u8; 16]);
impl FromStr for Int128 {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Int128(<[u8; 16]>::from_hex(&s).unwrap()))
    }
    type Err = FromHexError;
}
impl ToString for Int128 {
    fn to_string(&self) -> std::string::String {
        hex::encode(self.0)
    }
}

/// int256 8*[ int ] = Int256;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Int256(pub [u8; 32]);
impl FromStr for Int256 {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Int256(<[u8; 32]>::from_hex(&s).unwrap()))
    }
    type Err = FromHexError;
}
impl ToString for Int256 {
    fn to_string(&self) -> std::string::String {
        hex::encode(self.0)
    }
}

/// bytes data:string = Bytes;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "bytes", scheme = "lite.tl")]
pub struct Bytes{pub data:String}

/// tonNode.blockId workchain:int shard:long seqno:int = tonNode.BlockId;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockId {
    pub workchain: i32,
    // #[tl(with = "tl_shard")]
    pub shard: u64,
    pub seqno: u32,
}

/// tonNode.blockIdExt workchain:int shard:long seqno:int root_hash:int256 file_hash:int256 = tonNode.BlockIdExt;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
// #[tl(boxed, id = "tonNode.blockIdExt", scheme = "lite.tl")]
pub struct BlockIdExt {
    pub workchain: i32,
    pub shard: u64,
    pub seqno: u32,
    pub root_hash: Int256,
    pub file_hash: Int256,
}

/// tonNode.zeroStateIdExt workchain:int root_hash:int256 file_hash:int256 = tonNode.ZeroStateIdExt;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)] 
pub struct ZeroStateIdExt {
    pub workchain: i32,
    #[tl(size_hint = 32)]
    pub root_hash: Int256,
    #[tl(size_hint = 32)]
    pub file_hash: Int256,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, scheme = "lite.tl")]
pub enum Message {
    /// adnl.message.query query_id:int256 query:bytes = adnl.Message;
    #[tl(id = "adnl.message.query")]
    Query {
        query_id: Int256,
        query: Vec<u8>,
    },
    /// adnl.message.answer query_id:int256 answer:bytes = adnl.Message;
    #[tl(id = "adnl.message.answer")]
    Answer {
        query_id: Int256,
        answer: Vec<u8>,
    },
}

/// liteServer.error code:int message:string = liteServer.Error; 
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

/// liteServer.accountId workchain:int id:int256 = liteServer.AccountId;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.accountId", scheme = "lite.tl")]
pub struct AccountId {
    pub workchain: i32,
    pub id: Int256,
}

/// liteServer.masterchainInfo last:tonNode.blockIdExt state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.masterchainInfo", scheme = "lite.tl")]
pub struct MasterchainInfo {
    pub last: BlockIdExt,
    pub state_root_hash: Int256,
    pub init: ZeroStateIdExt,
}

/// liteServer.masterchainInfoExt mode:# version:int capabilities:long last:tonNode.blockIdExt last_utime:int now:int state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfoExt;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.masterchainInfoExt", scheme = "lite.tl")]
pub struct MasterchainInfoExt {
    #[tl(flags)]
    pub mode: (),
    pub version: i32,
    pub capabilities: i64,
    pub last: BlockIdExt,
    pub last_utime: i32,
    pub now: i32,
    pub state_root_hash: Int256,
    pub init: ZeroStateIdExt,
}

/// liteServer.currentTime now:int = liteServer.CurrentTime;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.currentTime", scheme = "lite.tl")]
pub struct CurrentTime {
    pub now: i32,
}

/// liteServer.version mode:# version:int capabilities:long now:int = liteServer.Version;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.version", scheme = "lite.tl")]
pub struct Version {
    pub mode: i32,
    pub version: i32,
    pub capabilities: i64,
    pub now: i32,
}

/// liteServer.blockData id:tonNode.blockIdExt data:bytes = liteServer.BlockData;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.blockData", scheme = "lite.tl")]
pub struct BlockData {
    pub id: BlockIdExt,
    pub data: Vec<u8>,
}

/// liteServer.blockState id:tonNode.blockIdExt root_hash:int256 file_hash:int256 data:bytes = liteServer.BlockState;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.blockState", scheme = "lite.tl")]
pub struct BlockState {
    pub id: BlockIdExt,
    pub root_hash: Int256,
    pub file_hash: Int256,
    pub data: Vec<u8>,
}

/// liteServer.blockHeader id:tonNode.blockIdExt mode:# header_proof:bytes = liteServer.BlockHeader;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.blockHeader", scheme = "lite.tl")]
pub struct BlockHeader {
    pub id: BlockIdExt,
    pub mode: i32,
    pub header_proof: Vec<u8>,
}

/// liteServer.sendMsgStatus status:int = liteServer.SendMsgStatus;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.sendMsgStatus", scheme = "lite.tl")]
pub struct SendMsgStatus {
    pub status: i32,
}

/// liteServer.accountState id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes proof:bytes state:bytes = liteServer.AccountState;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.accountState", scheme = "lite.tl")]
pub struct AccountState {
    pub id: BlockIdExt,
    pub shardblk: BlockIdExt,
    pub shard_proof: Vec<u8>,
    pub proof: Vec<u8>,
    pub state: Vec<u8>,
}

/// liteServer.runMethodResult mode:# id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:mode.0?bytes proof:mode.0?bytes state_proof:mode.1?bytes init_c7:mode.3?bytes lib_extras:mode.4?bytes exit_code:int result:mode.2?bytes = liteServer.RunMethodResult;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.runMethodResult", scheme = "lite.tl")]
pub struct RunMethodResult {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub shardblk: BlockIdExt,
    #[tl(flags_bit = "mode.0")]
    pub shard_proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.0")]
    pub proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.1")]
    pub state_proof: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.3")]
    pub init_c7: Option<Vec<u8>>,
    #[tl(flags_bit = "mode.4")]
    pub lib_extras: Option<Vec<u8>>,
    pub exit_code: i32,
    #[tl(flags_bit = "mode.2")]
    pub result: Option<Vec<u8>>,
}

/// liteServer.shardInfo id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes shard_descr:bytes = liteServer.ShardInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.shardInfo", scheme = "lite.tl")]
pub struct ShardInfo {
    pub id: BlockIdExt,
    pub shardblk: BlockIdExt,
    pub shard_proof: Vec<u8>,
    pub shard_descr: Vec<u8>,
}

/// liteServer.allShardsInfo id:tonNode.blockIdExt proof:bytes data:bytes = liteServer.AllShardsInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.allShardsInfo", scheme = "lite.tl")]
pub struct AllShardsInfo {
    pub id: BlockIdExt,
    pub proof: Vec<u8>,
    pub data: Vec<u8>,
}

/// liteServer.transactionInfo id:tonNode.blockIdExt proof:bytes transaction:bytes = liteServer.TransactionInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.transactionInfo", scheme = "lite.tl")]
pub struct TransactionInfo {
    pub id: BlockIdExt,
    pub proof: Vec<u8>,
    pub transaction: Vec<u8>,
}

/// liteServer.transactionList ids:(vector tonNode.blockIdExt) transactions:bytes = liteServer.TransactionList;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.transactionList", scheme = "lite.tl")]
pub struct TransactionList {
    pub ids: Vec<BlockIdExt>,
    pub transactions: Vec<u8>,
}

/// liteServer.transactionId mode:# account:mode.0?int256 lt:mode.1?long hash:mode.2?int256 = liteServer.TransactionId;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.transactionId", scheme = "lite.tl")]
pub struct TransactionId {
    #[tl(flags)]
    pub mode: (),
    #[tl(flags_bit = "mode.0")]
    pub account: Option<Int256>,
    #[tl(flags_bit = "mode.1")]
    pub lt: Option<i64>,
    #[tl(flags_bit = "mode.2")]
    pub hash: Option<Int256>,
}

/// liteServer.transactionId3 account:int256 lt:long = liteServer.TransactionId3;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.transactionId3", scheme = "lite.tl")]
pub struct TransactionId3 {
    pub account: Int256,
    pub lt: i64,
}

/// liteServer.blockTransactions id:tonNode.blockIdExt req_count:# incomplete:Bool ids:(vector liteServer.transactionId) proof:bytes = liteServer.BlockTransactions;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.blockTransactions", scheme = "lite.tl")]
pub struct BlockTransactions {
    pub id: BlockIdExt,
    #[tl(flags)]
    pub req_count: (),
    pub inclomplete: bool,
    pub ids: Vec<TransactionId>,
    pub proof: Vec<u8>,
}

/// liteServer.signature node_id_short:int256 signature:bytes = liteServer.Signature;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.signature", scheme = "lite.tl")]
pub struct Signature {
    pub node_id_short: Int256,
    pub signature: Vec<u8>,
}

/// liteServer.signatureSet validator_set_hash:int catchain_seqno:int signatures:(vector liteServer.signature) = liteServer.SignatureSet;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.signatureSet", scheme = "lite.tl")]
pub struct SignatureSet {
    pub validator_set_hash: i32,
    pub catchain_seqno: i32,
    pub signatures: Vec<Signature>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, scheme = "lite.tl")]
pub enum BlockLink {
    /// liteServer.blockLinkBack to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes proof:bytes state_proof:bytes = liteServer.BlockLink;
    #[tl(id = "liteServer.blockLinkBack")]
    BlockLinkBack {
        to_key_block: bool,
        from: BlockIdExt,
        to: BlockIdExt,
        dest_proof: Vec<u8>,
        proof: Vec<u8>,
        state_proof: Vec<u8>,
    },
    /// liteServer.blockLinkForward to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes config_proof:bytes signatures:liteServer.SignatureSet = liteServer.BlockLink;
    #[tl(id = "liteServer.blockLinkForward")]
    BlockLinkForward {
        to_key_block: bool,
        from: BlockIdExt,
        to: BlockIdExt,
        dest_proof: Vec<u8>,
        config_proof: Vec<u8>,
        signatures: SignatureSet,
    }
}

/// liteServer.partialBlockProof complete:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt steps:(vector liteServer.BlockLink) = liteServer.PartialBlockProof;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.partialBlockProof", scheme = "lite.tl")]
pub struct PartialBlockProof {
    pub complete: bool,
    pub from: BlockIdExt,
    pub to: BlockIdExt,
    pub steps: Vec<BlockLink>,
}

/// liteServer.configInfo mode:# id:tonNode.blockIdExt state_proof:bytes config_proof:bytes = liteServer.ConfigInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.configInfo", scheme = "lite.tl")]
pub struct ConfigInfo {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub state_proof: Vec<u8>,
    pub config_proof: Vec<u8>,
}

/// liteServer.validatorStats mode:# id:tonNode.blockIdExt count:int complete:Bool state_proof:bytes data_proof:bytes = liteServer.ValidatorStats;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.validatorStats", scheme = "lite.tl")]
pub struct ValidatorStats {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub count: i32,
    pub complete: bool,
    pub state_proof: Vec<u8>,
    pub data_proof: Vec<u8>,
}

/// liteServer.debug.verbosity value:int = liteServer.debug.Verbosity;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.debug.verbosity", scheme = "lite.tl")]
pub struct Verbosity {
    pub value: i32,
}

/// Functions

/// liteServer.getMasterchainInfo = liteServer.MasterchainInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getMasterchainInfo", scheme = "lite.tl")]
pub struct GetMasterchainInfo;

/// liteServer.getMasterchainInfoExt mode:# = liteServer.MasterchainInfoExt;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getMasterchainInfoExt", scheme = "lite.tl")]
pub struct GetMasterchainInfoExt {pub mode: i32}

/// liteServer.getTime = liteServer.CurrentTime;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getTime", scheme = "lite.tl")]
pub struct GetTime;

/// liteServer.getVersion = liteServer.Version;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getVersion", scheme = "lite.tl")]
pub struct GetVersion;

/// liteServer.getBlock id:tonNode.blockIdExt = liteServer.BlockData;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getBlock", scheme = "lite.tl")]
pub struct GetBlock {
    pub id: BlockIdExt,
}

/// liteServer.getState id:tonNode.blockIdExt = liteServer.BlockState;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getState", scheme = "lite.tl")]
pub struct GetState {
    pub id: BlockIdExt,
}

/// liteServer.getBlockHeader id:tonNode.blockIdExt mode:# = liteServer.BlockHeader;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getBlockHeader", scheme = "lite.tl")]
pub struct GetBlockHeader {
    pub id: BlockIdExt,
    #[tl(flags)]
    pub mode: (),
}

/// liteServer.sendMessage body:bytes = liteServer.SendMsgStatus;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.sendMessage", scheme = "lite.tl")]
pub struct SendMessage {
    pub body: Vec<u8>,
}

/// liteServer.getAccountState id:tonNode.blockIdExt account:liteServer.accountId = liteServer.AccountState;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getAccountState", scheme = "lite.tl")]
pub struct GetAccountState {
    pub id: BlockIdExt,
    pub account: AccountId,
}

/// liteServer.runSmcMethod mode:# id:tonNode.blockIdExt account:liteServer.accountId method_id:long params:bytes = liteServer.RunMethodResult;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.runSmcMethod", scheme = "lite.tl")]
pub struct RunSmcMethod {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub account: AccountId,
    pub method_id: i64,
    pub params: Vec<u8>,
}

/// liteServer.getShardInfo id:tonNode.blockIdExt workchain:int shard:long exact:Bool = liteServer.ShardInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getShardInfo", scheme = "lite.tl")]
pub struct GetShardInfo {
    pub id: BlockIdExt,
    pub workchain: i32,
    pub shard: i64,
    pub exact: bool,
}
/// liteServer.getAllShardsInfo id:tonNode.blockIdExt = liteServer.AllShardsInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getAllShardsInfo", scheme = "lite.tl")]
pub struct GetAllShardsInfo {
    pub id: BlockIdExt,
}

/// liteServer.getOneTransaction id:tonNode.blockIdExt account:liteServer.accountId lt:long = liteServer.TransactionInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getOneTransaction", scheme = "lite.tl")]
pub struct GetOneTransaction {
    pub id: BlockIdExt,
    pub account: AccountId,
    pub lt: i64,
}

/// liteServer.getTransactions count:# account:liteServer.accountId lt:long hash:int256 = liteServer.TransactionList;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getTransactions", scheme = "lite.tl")]
pub struct GetTransactions {
    pub count: i32,
    pub account: AccountId,
    pub lt: i64,
    pub hash: Int256,
}

/// liteServer.lookupBlock mode:# id:tonNode.blockId lt:mode.1?long utime:mode.2?int = liteServer.BlockHeader;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.lookupBlock", scheme = "lite.tl")]
pub struct LookupBlock {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockId,
    #[tl(flags_bit = "mode.1")]
    pub lt: Option<i64>,
    #[tl(flags_bit = "mode.2")]
    pub utime: Option<i32>,
}

/// liteServer.listBlockTransactions id:tonNode.blockIdExt mode:# count:# after:mode.7?liteServer.transactionId3 reverse_order:mode.6?true want_proof:mode.5?true = liteServer.BlockTransactions;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.listBlockTransactions", scheme = "lite.tl")]
pub struct ListBlockTransactions {
    pub id: BlockIdExt,
    #[tl(flags)]
    pub mode: (),
    pub count: i32,
    #[tl(flags_bit = "mode.7")]
    pub after: Option<TransactionId3>,
    #[tl(flags_bit = "mode.6")]
    pub reverse_order: Option<True>,
    #[tl(flags_bit = "mode.5")]
    pub want_proof: Option<True>,
}

/// liteServer.getBlockProof mode:# known_block:tonNode.blockIdExt target_block:mode.0?tonNode.blockIdExt = liteServer.PartialBlockProof;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getBlockProof", scheme = "lite.tl")]
pub struct GetBlockProof {
    #[tl(flags)]
    pub mode: (),
    pub known_block: BlockIdExt,
    #[tl(flags_bit = "mode.0")]
    pub target_block: Option<BlockIdExt>,
}

/// liteServer.getConfigAll mode:# id:tonNode.blockIdExt = liteServer.ConfigInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getConfigAll", scheme = "lite.tl")]
pub struct GetConfigAll {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
}

/// liteServer.getConfigParams mode:# id:tonNode.blockIdExt param_list:(vector int) = liteServer.ConfigInfo;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.getConfigParams", scheme = "lite.tl")]
pub struct GetConfigParams {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub param_list: Vec<i32>,
}

/// liteServer.getValidatorStats#091a58bc mode:# id:tonNode.blockIdExt limit:int start_after:mode.0?int256 modified_after:mode.2?int = liteServer.ValidatorStats;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = 0x091a58bc)]
pub struct GetValidatorStats {
    #[tl(flags)]
    pub mode: (),
    pub id: BlockIdExt,
    pub limit: i32,
    #[tl(flags_bit = "mode.0")]
    pub start_after: Option<Int256>,
    #[tl(flags_bit = "mode.2")]
    pub modified_after: Option<i32>,

}

/// liteServer.query data:bytes = Object;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed, id = "liteServer.query", scheme = "lite.tl")]
pub struct Query{pub data: Vec<u8>}


#[cfg(test)]
mod tests {
    use crate::scheme::*;
    use hex::FromHex;
    use tl_proto;
    #[test]
    fn block_id_test() {
        let bytes = tl_proto::serialize(&BlockId {workchain: 0, shard: 0x8000000000000000, seqno: 13131131});
        println!("{:?}", bytes);
        let decoded = tl_proto::deserialize(&bytes).unwrap();
        assert!(matches!(
            decoded,
            BlockId {workchain: 0, shard: 0x8000000000000000, seqno: 13131131}
        ));
    }
    #[test]
    fn int256_test() {
        let hex_hash = "7f43835181544d3721196153f912226625568035627bdc5df827c983a4965cae";
        let encoded_hash = <[u8; 32]>::from_hex(&hex_hash).unwrap();
        println!("{:?}", tl_proto::serialize(Int256(encoded_hash)));
        let _check: [u8; 32] = [127, 67, 131, 81, 129, 84, 77, 55, 33, 25, 97, 83, 249, 18, 34, 102, 37, 86, 128, 53, 98, 123, 220, 93, 248, 39, 201, 131, 164, 150, 92, 174];
        assert!(matches!(tl_proto::serialize(Int256(encoded_hash)), _check));
    }
    // #[test]
    // fn blockId_test() {
    //     let 
    // }
}

