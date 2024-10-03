use core::fmt;
use std::{fmt::Display, str::FromStr};

use derivative::Derivative;
use hex::FromHex;
use tl_proto::{TlRead, TlWrite};
use super::utils::*;

/// true = True;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct True;

/// string ? = String;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct String(#[derivative(Debug(format_with = "fmt_string"))] Vec<u8>);

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            std::string::String::from_utf8(self.0.clone()).unwrap()
        )
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}

impl String {
    pub fn new(str: std::string::String) -> Self {
        Self(str.into_bytes())
    }
}

/// int256 8*[ int ] = Int256;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Int256(#[derivative(Debug(format_with = "fmt_bytes"))] pub [u8; 32]);

impl FromStr for Int256 {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
    type Err = hex::FromHexError;
}

impl Display for Int256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl Int256 {
    pub fn to_hex(&self) -> std::string::String {
        hex::encode(self.0)
    }

    pub fn from_hex(s: &str) -> Result<Self, hex::FromHexError> {
        Ok(Int256(<[u8; 32]>::from_hex(s)?))
    }

    pub fn random() -> Self {
        Self(rand::random())
    }
}

/// tonNode.blockId workchain:int shard:long seqno:int = tonNode.BlockId;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct BlockId {
    pub workchain: i32,
    pub shard: u64,
    pub seqno: u32,
}

/// tonNode.blockIdExt workchain:int shard:long seqno:int root_hash:int256 file_hash:int256 = tonNode.BlockIdExt;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockIdExt {
    pub workchain: i32,
    pub shard: u64,
    pub seqno: u32,
    pub root_hash: Int256,
    pub file_hash: Int256,
}

impl fmt::Display for BlockIdExt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{:X},{}):{}:{}", self.workchain, self.shard, self.seqno, self.root_hash.to_string(), self.file_hash.to_string())
    }
}

/// liteServer.accountId workchain:int id:int256 = liteServer.AccountId;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct AccountId {
    pub workchain: i32,
    pub id: Int256,
}

/// liteServer.transactionId3 account:int256 lt:long = liteServer.TransactionId3;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
// #[tl(boxed, id = "liteServer.transactionId3", scheme_inline = r##"liteServer.transactionId3 account:int256 lt:long = liteServer.TransactionId3;"##)]
pub struct TransactionId3 {
    pub account: Int256,
    pub lt: u64,
}

/// liteServer.signature node_id_short:int256 signature:bytes = liteServer.Signature;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
// #[tl(boxed, id = "liteServer.signature", scheme_inline = r##"liteServer.signature node_id_short:int256 signature:bytes = liteServer.Signature;"##)]
pub struct Signature {
    pub node_id_short: Int256,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub signature: Vec<u8>,
}

/// liteServer.signatureSet validator_set_hash:int catchain_seqno:int signatures:(vector liteServer.signature) = liteServer.SignatureSet;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(
    boxed,
    id = "liteServer.signatureSet",
    scheme_inline = r##"liteServer.signatureSet validator_set_hash:int catchain_seqno:int signatures:(vector liteServer.signature) = liteServer.SignatureSet;"##
)]
pub struct SignatureSet {
    pub validator_set_hash: u32,
    pub catchain_seqno: u32,
    pub signatures: Vec<Signature>,
}

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(
    boxed,
    scheme_inline = r##"liteServer.blockLinkBack to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes proof:bytes state_proof:bytes = liteServer.BlockLink;
        liteServer.blockLinkForward to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes config_proof:bytes signatures:liteServer.SignatureSet = liteServer.BlockLink;"##
)]
pub enum BlockLink {
    /// liteServer.blockLinkBack to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes proof:bytes state_proof:bytes = liteServer.BlockLink;
    #[tl(id = "liteServer.blockLinkBack")]
    BlockLinkBack {
        to_key_block: bool,
        from: BlockIdExt,
        to: BlockIdExt,
        #[derivative(Debug(format_with = "fmt_bytes"))]
        dest_proof: Vec<u8>,
        #[derivative(Debug(format_with = "fmt_bytes"))]
        proof: Vec<u8>,
        #[derivative(Debug(format_with = "fmt_bytes"))]
        state_proof: Vec<u8>,
    },
    /// liteServer.blockLinkForward to_key_block:Bool from:tonNode.blockIdExt to:tonNode.blockIdExt dest_proof:bytes config_proof:bytes signatures:liteServer.SignatureSet = liteServer.BlockLink;
    #[tl(id = "liteServer.blockLinkForward")]
    BlockLinkForward {
        to_key_block: bool,
        from: BlockIdExt,
        to: BlockIdExt,
        #[derivative(Debug(format_with = "fmt_bytes"))]
        dest_proof: Vec<u8>,
        #[derivative(Debug(format_with = "fmt_bytes"))]
        config_proof: Vec<u8>,
        signatures: SignatureSet,
    },
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

/// liteServer.transactionId mode:# account:mode.0?int256 lt:mode.1?long hash:mode.2?int256 = liteServer.TransactionId;
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
}

/// liteServer.libraryEntry hash:int256 data:bytes = liteServer.LibraryEntry;
#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
pub struct LibraryEntry {
    pub hash: Int256,
    #[derivative(Debug(format_with = "fmt_bytes"))]
    pub data: Vec<u8>,
}
