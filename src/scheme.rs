#![allow(dead_code)]

use tl_proto::{TlRead, TlWrite};
use std::str::FromStr;
use hex::{FromHex, FromHexError};

/// true = True;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "true", scheme = "lite.tl")]
pub struct True;

/// string ? = String;
#[derive(TlRead, TlWrite, Debug)]
pub struct String(pub Vec<u8>);

/// int128 4*[ int ] = Int128;
#[derive(TlRead, TlWrite, Debug)]
pub struct Int128([u8; 16]);

/// int256 8*[ int ] = Int256;
#[derive(TlRead, TlWrite, Debug)]
pub struct Int256([u8; 32]);

impl Int256 {
    pub const fn with_array(data: [u8; 32]) -> Self {
        Self(data)
    }
}
impl FromStr for Int256 {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Int256(<[u8; 32]>::from_hex(&s).unwrap()))
    }

    type Err = FromHexError;
}

/// bytes data:string = Bytes;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "bytes", scheme = "lite.tl")]
pub struct Bytes{pub data:String}

/// tonNode.blockId workchain:int shard:long seqno:int = tonNode.BlockId;
#[derive(TlRead, TlWrite, Debug)]
pub struct BlockId {
    workchain: i32,
    // #[tl(with = "tl_shard")]
    shard: u64,
    seqno: u32,
}

/// tonNode.blockIdExt workchain:int shard:long seqno:int root_hash:int256 file_hash:int256 = tonNode.BlockIdExt;
#[derive(TlRead, TlWrite, Debug)]
pub struct BlockIdExt {
    workchain: i32,
    shard: u64,
    seqno: u32,
    root_hash: Int256,
    file_hash: Int256,
}

/// tonNode.zeroStateIdExt workchain:int root_hash:int256 file_hash:int256 = tonNode.ZeroStateIdExt;
#[derive(TlRead, TlWrite, Debug)] 
pub struct ZeroStateIdExt {
    workchain: i32,
    #[tl(size_hint = 32)]
    root_hash: Int256,
    #[tl(size_hint = 32)]
    file_hash: Int256,
}

#[derive(TlRead, TlWrite, Debug)]
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
#[derive(TlRead, TlWrite, Debug)]
pub struct Error {
    code: i32,
    message: String,
}

/// liteServer.accountId workchain:int id:int256 = liteServer.AccountId;
#[derive(TlRead, TlWrite, Debug)]
pub struct AccountId {
    workchain: i32,
    id: Int256,
}

/// liteServer.masterchainInfo last:tonNode.blockIdExt state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfo;
#[derive(TlRead, TlWrite, Debug)]
pub struct MasterchainInfo {
    last: BlockIdExt,
    state_root_hash: Int256,
    init: ZeroStateIdExt,
}

/// liteServer.masterchainInfoExt mode:# version:int capabilities:long last:tonNode.blockIdExt last_utime:int now:int state_root_hash:int256 init:tonNode.zeroStateIdExt = liteServer.MasterchainInfoExt;
#[derive(TlRead, TlWrite, Debug)]
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

/// liteServer.currentTime now:int = liteServer.CurrentTime;
#[derive(TlRead, TlWrite, Debug)]
pub struct CurrentTime {
    now: i32,
}

/// liteServer.version mode:# version:int capabilities:long now:int = liteServer.Version;
#[derive(TlRead, TlWrite, Debug)]
// #[tl(boxed, id = "liteServer.version", scheme = "lite.tl")]
pub struct Version {
    #[tl(flags)]
    mode: (),
    version: i32,
    capabilities: i64,
    now: i32,
}

/// liteServer.blockData id:tonNode.blockIdExt data:bytes = liteServer.BlockData;
#[derive(TlRead, TlWrite, Debug)]
pub struct BlockData {
    id: BlockIdExt,
    data: Vec<u8>,
}

/// liteServer.blockState id:tonNode.blockIdExt root_hash:int256 file_hash:int256 data:bytes = liteServer.BlockState;
#[derive(TlRead, TlWrite, Debug)]
pub struct BlockState {
    id: BlockIdExt,
    root_hash: Int256,
    file_hash: Int256,
    data: Vec<u8>,
}

/// liteServer.blockHeader id:tonNode.blockIdExt mode:# header_proof:bytes = liteServer.BlockHeader;
#[derive(TlRead, TlWrite, Debug)]
pub struct BlockHeader {
    id: BlockIdExt,
    mode: i32,
    header_proof: Vec<u8>,
}

/// liteServer.sendMsgStatus status:int = liteServer.SendMsgStatus;
#[derive(TlRead, TlWrite, Debug)]
pub struct SendMsgStatus {
    status: i32,
}

/// liteServer.accountState id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes proof:bytes state:bytes = liteServer.AccountState;
#[derive(TlRead, TlWrite, Debug)]
pub struct AccountState {
    id: BlockIdExt,
    shardblk: BlockIdExt,
    shard_proof: Vec<u8>,
    proof: Vec<u8>,
    state: Vec<u8>,
}

/// liteServer.runMethodResult mode:# id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:mode.0?bytes proof:mode.0?bytes state_proof:mode.1?bytes init_c7:mode.3?bytes lib_extras:mode.4?bytes exit_code:int result:mode.2?bytes = liteServer.RunMethodResult;
#[derive(TlRead, TlWrite, Debug)]
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

/// liteServer.shardInfo id:tonNode.blockIdExt shardblk:tonNode.blockIdExt shard_proof:bytes shard_descr:bytes = liteServer.ShardInfo;
#[derive(TlRead, TlWrite, Debug)]
pub struct ShardInfo {
    id: BlockIdExt,
    shardblk: BlockIdExt,
    shard_proof: Vec<u8>,
    shard_descr: Vec<u8>,
}

/// liteServer.allShardsInfo id:tonNode.blockIdExt proof:bytes data:bytes = liteServer.AllShardsInfo;
#[derive(TlRead, TlWrite, Debug)]
pub struct AllShardsInfo {
    id: BlockIdExt,
    proof: Vec<u8>,
    data: Vec<u8>,
}

/// liteServer.transactionInfo id:tonNode.blockIdExt proof:bytes transaction:bytes = liteServer.TransactionInfo;
#[derive(TlRead, TlWrite, Debug)]
pub struct TransactionInfo {
    id: BlockIdExt,
    proof: Vec<u8>,
    transaction: Vec<u8>,
}

/// liteServer.transactionList ids:(vector tonNode.blockIdExt) transactions:bytes = liteServer.TransactionList;
#[derive(TlRead, TlWrite, Debug)]
pub struct TransactionList {
    ids: Vec<BlockIdExt>,
    transactions: Vec<u8>,
}

/// liteServer.transactionId mode:# account:mode.0?int256 lt:mode.1?long hash:mode.2?int256 = liteServer.TransactionId;
#[derive(TlRead, TlWrite, Debug)]
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

/// liteServer.transactionId3 account:int256 lt:long = liteServer.TransactionId3;
#[derive(TlRead, TlWrite, Debug)]
pub struct TransactionId3 {
    account: Int256,
    lt: i64,
}

/// liteServer.blockTransactions id:tonNode.blockIdExt req_count:# incomplete:Bool ids:(vector liteServer.transactionId) proof:bytes = liteServer.BlockTransactions;
#[derive(TlRead, TlWrite, Debug)]
pub struct BlockTransactions {
    id: BlockIdExt,
    #[tl(flags)]
    req_count: (),
    inclomplete: bool,
    ids: Vec<TransactionId>,
    proof: Vec<u8>,
}

/// liteServer.signature node_id_short:int256 signature:bytes = liteServer.Signature;
#[derive(TlRead, TlWrite, Debug)]
pub struct Signature {
    node_id_short: Int256,
    signature: Vec<u8>,
}

/// liteServer.signatureSet validator_set_hash:int catchain_seqno:int signatures:(vector liteServer.signature) = liteServer.SignatureSet;
#[derive(TlRead, TlWrite, Debug)]
pub struct SignatureSet {
    validator_set_hash: i32,
    catchain_seqno: i32,
    signatures: Vec<Signature>,
}

#[derive(TlRead, TlWrite, Debug)]
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
        state_proof:Vec<u8>,
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
#[derive(TlRead, TlWrite, Debug)]
pub struct PartialBlockProof {
    complete: bool,
    from: BlockIdExt,
    to: BlockIdExt,
    steps: Vec<BlockLink>,
}

/// liteServer.configInfo mode:# id:tonNode.blockIdExt state_proof:bytes config_proof:bytes = liteServer.ConfigInfo;
#[derive(TlRead, TlWrite, Debug)]
pub struct ConfigInfo {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
    state_proof: Vec<u8>,
    config_proof: Vec<u8>,
}

/// liteServer.validatorStats mode:# id:tonNode.blockIdExt count:int complete:Bool state_proof:bytes data_proof:bytes = liteServer.ValidatorStats;
#[derive(TlRead, TlWrite, Debug)]
pub struct ValidatorStats {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
    count: i32,
    complete: bool,
    state_proof: Vec<u8>,
    data_proof: Vec<u8>,
}

/// liteServer.debug.verbosity value:int = liteServer.debug.Verbosity;
#[derive(TlRead, TlWrite, Debug)]
pub struct Verbosity {
    value: i32,
}

/// Functions

/// liteServer.getMasterchainInfo = liteServer.MasterchainInfo;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getMasterchainInfo", scheme = "lite.tl")]
pub struct GetMasterchainInfo;

/// liteServer.getMasterchainInfoExt mode:# = liteServer.MasterchainInfoExt;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getMasterchainInfoExt", scheme = "lite.tl")]
pub struct GetMasterchainInfoExt;

/// liteServer.getTime = liteServer.CurrentTime;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getTime", scheme = "lite.tl")]
pub struct GetTime;

/// liteServer.getVersion = liteServer.Version;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getVersion", scheme = "lite.tl")]
pub struct GetVersion;

/// liteServer.getBlock id:tonNode.blockIdExt = liteServer.BlockData;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getBlock", scheme = "lite.tl")]
pub struct GetBlock {
    id: BlockIdExt,
}

/// liteServer.getState id:tonNode.blockIdExt = liteServer.BlockState;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getState", scheme = "lite.tl")]
pub struct GetState {
    id: BlockIdExt,
}

/// liteServer.getBlockHeader id:tonNode.blockIdExt mode:# = liteServer.BlockHeader;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getBlockHeader", scheme = "lite.tl")]
pub struct GetBlockHeader {
    id: BlockIdExt,
    #[tl(flags)]
    mode: (),
}

/// liteServer.sendMessage body:bytes = liteServer.SendMsgStatus;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.sendMessage", scheme = "lite.tl")]
pub struct SendMessage {
    body: Vec<u8>,
}

/// liteServer.getAccountState id:tonNode.blockIdExt account:liteServer.accountId = liteServer.AccountState;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getAccountState", scheme = "lite.tl")]
pub struct GetAccountState {
    id: BlockIdExt,
    account: AccountId,
}

/// liteServer.runSmcMethod mode:# id:tonNode.blockIdExt account:liteServer.accountId method_id:long params:bytes = liteServer.RunMethodResult;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.runSmcMethod", scheme = "lite.tl")]
pub struct RunSmcMethod {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
    account: AccountId,
    method_id: i64,
    params: Vec<u8>,
}

/// liteServer.getShardInfo id:tonNode.blockIdExt workchain:int shard:long exact:Bool = liteServer.ShardInfo;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getShardInfo", scheme = "lite.tl")]
pub struct GetShardInfo {
    id: BlockIdExt,
    workchain: i32,
    shard: i64,
    exact: bool,
}
/// liteServer.getAllShardsInfo id:tonNode.blockIdExt = liteServer.AllShardsInfo;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getAllShardsInfo", scheme = "lite.tl")]
pub struct GetAllShardsInfo {
    id: BlockIdExt,
}

/// liteServer.getOneTransaction id:tonNode.blockIdExt account:liteServer.accountId lt:long = liteServer.TransactionInfo;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getOneTransaction", scheme = "lite.tl")]
pub struct GetOneTransaction {
    id: BlockIdExt,
    account: AccountId,
    lt: i64,
}

/// liteServer.getTransactions count:# account:liteServer.accountId lt:long hash:int256 = liteServer.TransactionList;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getTransactions", scheme = "lite.tl")]
pub struct GetTransactions {
    #[tl(flags)]
    count: (),
    account: AccountId,
    lt: i64,
    hash: Int256,
}

/// liteServer.lookupBlock mode:# id:tonNode.blockId lt:mode.1?long utime:mode.2?int = liteServer.BlockHeader;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.lookupBlock", scheme = "lite.tl")]
pub struct LookupBlock {
    #[tl(flags)]
    mode: (),
    id: BlockId,
    #[tl(flags_bit = "mode.1")]
    lt: Option<i64>,
    #[tl(flags_bit = "mode.2")]
    utime: Option<i32>,
}

/// liteServer.listBlockTransactions id:tonNode.blockIdExt mode:# count:# after:mode.7?liteServer.transactionId3 reverse_order:mode.6?true want_proof:mode.5?true = liteServer.BlockTransactions;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.listBlockTransactions", scheme = "lite.tl")]
pub struct ListBlockTransactions {
    id: BlockIdExt,
    #[tl(flags)]
    mode: (),
    #[tl(flags)]
    count: (),
    #[tl(flags_bit = "mode.7")]
    after: Option<TransactionId3>,
    #[tl(flags_bit = "mode.6")]
    reverse_order: Option<True>,
    #[tl(flags_bit = "mode.5")]
    want_proof: Option<True>,
}

/// liteServer.getBlockProof mode:# known_block:tonNode.blockIdExt target_block:mode.0?tonNode.blockIdExt = liteServer.PartialBlockProof;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getBlockProof", scheme = "lite.tl")]
pub struct GetBlockProof {
    #[tl(flags)]
    mode: (),
    known_block: BlockIdExt,
    #[tl(flags_bit = "mode.0")]
    target_block: Option<BlockIdExt>,
}

/// liteServer.getConfigAll mode:# id:tonNode.blockIdExt = liteServer.ConfigInfo;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getConfigAll", scheme = "lite.tl")]
pub struct GetConfigAll {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
}

/// liteServer.getConfigParams mode:# id:tonNode.blockIdExt param_list:(vector int) = liteServer.ConfigInfo;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.getConfigParams", scheme = "lite.tl")]
pub struct GetConfigParams {
    #[tl(flags)]
    mode: (),
    id: BlockIdExt,
    param_list: Vec<i32>,
}

/// liteServer.getValidatorStats#091a58bc mode:# id:tonNode.blockIdExt limit:int start_after:mode.0?int256 modified_after:mode.2?int = liteServer.ValidatorStats;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = 0x091a58bc)]
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

/// liteServer.query data:bytes = Object;
#[derive(TlRead, TlWrite, Debug)]
#[tl(boxed, id = "liteServer.query", scheme = "lite.tl")]
pub struct Query{pub data: Vec<u8>}


#[cfg(test)]
mod tests {
    use crate::scheme::*;
    use hex::{FromHex};
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
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: InvalidHexCharacter { c: 'h', index: 1 }")]
    fn int256_panic() {
        let hex_hash = "7h43835181544d3721196153f912226625568035627bdc5df827c983a4965cae";
        let _a = Int256::from_str(hex_hash).unwrap();
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

