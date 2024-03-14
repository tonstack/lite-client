use adnl::AdnlError;
use tl_proto::TlError;
use tower_service::Service;
use thiserror::Error;

use crate::tl_types::*;

pub enum LiteRequest {
    GetMasterchainInfo,
    GetMasterchainInfoExt(GetMasterchainInfoExt),
    GetTime,
    GetVersion,
    GetBlock(GetBlock),
    GetState(GetState),
    GetBlockHeader(GetBlockHeader),
    SendMessage(SendMessage),
    GetAccountState(GetAccountState),
    RunSmcMethod(RunSmcMethod),
    GetShardInfo(GetShardInfo),
    GetAllShardsInfo(GetAllShardsInfo),
    GetOneTransaction(GetOneTransaction),
    GetTransactions(GetTransactions),
    LookupBlock(LookupBlock),
    ListBlockTransactions(ListBlockTransactions),
    GetBlockProof(GetBlockProof),
    GetConfigAll(GetConfigAll),
    GetConfigParams(GetConfigParams),
    GetValidatorStats(GetValidatorStats),
}

impl LiteRequest {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            LiteRequest::GetMasterchainInfo => tl_proto::serialize(GetMasterchainInfo),
            LiteRequest::GetMasterchainInfoExt(req) => tl_proto::serialize(req),
            LiteRequest::GetTime => tl_proto::serialize(GetTime),
            LiteRequest::GetVersion => tl_proto::serialize(GetVersion),
            LiteRequest::GetBlock(req) => tl_proto::serialize(req),
            LiteRequest::GetState(req) => tl_proto::serialize(req),
            LiteRequest::GetBlockHeader(req) => tl_proto::serialize(req),
            LiteRequest::SendMessage(req) => tl_proto::serialize(req),
            LiteRequest::GetAccountState(req) => tl_proto::serialize(req),
            LiteRequest::RunSmcMethod(req) => tl_proto::serialize(req),
            LiteRequest::GetShardInfo(req) => tl_proto::serialize(req),
            LiteRequest::GetAllShardsInfo(req) => tl_proto::serialize(req),
            LiteRequest::GetOneTransaction(req) => tl_proto::serialize(req),
            LiteRequest::GetTransactions(req) => tl_proto::serialize(req),
            LiteRequest::LookupBlock(req) => tl_proto::serialize(req),
            LiteRequest::ListBlockTransactions(req) => tl_proto::serialize(req),
            LiteRequest::GetBlockProof(req) => tl_proto::serialize(req),
            LiteRequest::GetConfigAll(req) => tl_proto::serialize(req),
            LiteRequest::GetConfigParams(req) => tl_proto::serialize(req),
            LiteRequest::GetValidatorStats(req) => tl_proto::serialize(req),
        }
    }

    pub fn deserialize(buf: &[u8]) -> Result<LiteRequest, LiteError> {
        let data = tl_proto::deserialize::<Message>(buf).map_err(|e| LiteError::TlError(e))?;
            let (query_id, answer) = match data {
                Message::Answer { query_id, answer } => (query_id, answer),
                msg => {
                    log::error!("Got wrong adnl.Message type from server, expected adnl.message.answer:\n{:#?}", msg);
                    return Err(LiteError::UnexpectedMessage);
                }
            };

            tl_proto::deserialize(packet)

            // deserialize actual answer or deserialize error if any
            let result = tl_proto::deserialize::<U>(response).or_else(|e| {
                if let TlError::UnknownConstructor = e {
                    let err = tl_proto::deserialize::<tl_types::Error>(response)
                        .map_err(|e| LiteError::TlError(e))?;
                    log::debug!("liteapi response error: {:?}", err);
                    Err(LiteError::ServerError(err))
                } else {
                    Err(LiteError::TlError(e))
                }
            })?;
    }
}

pub enum LiteResponse {
    MasterchainInfo(MasterchainInfo),
    MasterchainInfoExt(MasterchainInfoExt),
    CurrentTime(CurrentTime),
    Version(Version),
    BlockData(BlockData),
    BlockState(BlockState),
    BlockHeader(BlockHeader),
    SendMsgStatus(SendMsgStatus),
    AccountState(AccountState),
    RunMethodResult(RunMethodResult),
    ShardInfo(ShardInfo),
    AllShardsInfo(AllShardsInfo),
    TransactionInfo(TransactionInfo),
    TransactionList(TransactionList),
    BlockTransactions(BlockTransactions),
    PartialBlockProof(PartialBlockProof),
    ConfigInfo(ConfigInfo),
    ValidatorStats(ValidatorStats),
}

impl LiteResponse {
    pub fn serialize(&self) -> Vec<u8> {
        match self {
            LiteResponse::MasterchainInfo(req) => tl_proto::serialize(req),
            LiteResponse::MasterchainInfoExt(req) => tl_proto::serialize(req),
            LiteResponse::CurrentTime(req) => tl_proto::serialize(req),
            LiteResponse::Version(req) => tl_proto::serialize(req),
            LiteResponse::BlockData(req) => tl_proto::serialize(req),
            LiteResponse::BlockState(req) => tl_proto::serialize(req),
            LiteResponse::BlockHeader(req) => tl_proto::serialize(req),
            LiteResponse::SendMsgStatus(req) => tl_proto::serialize(req),
            LiteResponse::AccountState(req) => tl_proto::serialize(req),
            LiteResponse::RunMethodResult(req) => tl_proto::serialize(req),
            LiteResponse::ShardInfo(req) => tl_proto::serialize(req),
            LiteResponse::AllShardsInfo(req) => tl_proto::serialize(req),
            LiteResponse::TransactionInfo(req) => tl_proto::serialize(req),
            LiteResponse::TransactionList(req) => tl_proto::serialize(req),
            LiteResponse::BlockTransactions(req) => tl_proto::serialize(req),
            LiteResponse::PartialBlockProof(req) => tl_proto::serialize(req),
            LiteResponse::ConfigInfo(req) => tl_proto::serialize(req),
            LiteResponse::ValidatorStats(req) => tl_proto::serialize(req),
        }
    }
}

#[derive(Debug, Error)]
pub enum LiteError {
    #[error("Liteserver error")]
    ServerError(Error),
    #[error("TL parsing error")]
    TlError(TlError),
    #[error("Unexpected TL message")]
    UnexpectedMessage,
    #[error("ADNL error")]
    AdnlError(AdnlError),
    #[error(transparent)]
    OtherError(#[from] Error),
}

pub trait LiteService: Service<LiteRequest, Response = LiteResponse, Error = LiteError>{}
