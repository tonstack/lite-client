pub mod config;

#[cfg(test)]
mod tests;
mod scheme;

pub use private::LiteClient;
pub use private::Result;
pub use private::DeserializeError;

mod private {
    use std::error::Error;
    use ton_api::ton::TLObject;
    use ton_api::ton::lite_server as lite_result;
    use pretty_hex::PrettyHex;
    use std::fmt::{Display, Formatter};
    use std::net::TcpStream;
    use x25519_dalek::StaticSecret;
    use adnl::{AdnlClient, AdnlBuilder};
    use rand::prelude::SliceRandom;
    use crate::config::ConfigGlobal;
    use crate::scheme;
    use tl_proto::{TlWrite, Bare, TlResult, TlRead};


    #[derive(Debug)]
    pub struct DeserializeError {
        object: TLObject,
    }

    impl Display for DeserializeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Deserialize error, can't downcast {:?}", self.object)
        }
    }

    impl Error for DeserializeError {}

    #[derive(Debug)]
    pub struct LiteError(lite_result::Error);

    impl Into<lite_result::Error> for LiteError {
        fn into(self) -> lite_result::Error {
            self.0
        }
    }

    impl From<lite_result::Error> for LiteError {
        fn from(e: lite_result::Error) -> Self {
            Self(e)
        }
    }

    impl Display for LiteError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Server error [code={}]: {}", self.0.code(), self.0.message())
        }
    }

    impl Error for LiteError {}

    pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

    pub struct LiteClient {
        client: AdnlClient<TcpStream>,
    }

    impl LiteClient {
        pub fn connect(config_json: &str) -> Result<Self> {
            let config: ConfigGlobal = serde_json::from_str(config_json)?;
            let ls = config.liteservers.choose(&mut rand::thread_rng()).unwrap();
            let local_secret = StaticSecret::new(rand::rngs::OsRng);
            let transport = TcpStream::connect(ls.socket_addr())?;
            let client = AdnlBuilder::with_random_aes_params(&mut rand::rngs::OsRng)
                .perform_ecdh(local_secret, ls.id.clone())
                .perform_handshake(transport).map_err(|e| format!("{:?}", e))?;
            Ok(Self { client })
        }
        pub fn lite_query<'tl, T, U>(&mut self, request: T, response: &'tl mut Vec<u8>) -> TlResult<U> 
        where 
            T: TlWrite,
            U: TlRead<'tl, Repr = Bare> 
        {
            let mut message = tl_proto::serialize(scheme::Message::Query { 
                query_id: (scheme::Int256::with_array(rand::random())), 
                query: (tl_proto::serialize(scheme::Query{data: (tl_proto::serialize(request))})) 
            });
            
            log::debug!("Sending query:\n{:?}", &message.hex_dump());
            self.client.send(&mut message, &mut rand::random())
                .map_err(|e| format!("{:?}", e)).unwrap();
            log::debug!("Query sent");
            self.client.receive::<_, 8192>(response)
                .map_err(|e| format!("{:?}", e)).unwrap();
            log::debug!("Received:\n{:?}", &response.hex_dump());
            let data = tl_proto::deserialize::<scheme::Message>(response).unwrap();
            // Ok(data)
            if let scheme::Message::Answer { query_id: _, answer} = data {
                *response = answer;
            }
            else {panic!();}
            tl_proto::deserialize::<U>(response)
        }

        pub fn get_masterchain_info(&mut self) -> TlResult<scheme::MasterchainInfo> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(scheme::GetMasterchainInfo, &mut response) as TlResult<scheme::MasterchainInfo> 
        }

        pub fn get_version(&mut self) -> TlResult<scheme::Version> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(scheme::GetVersion, &mut response) as TlResult<scheme::Version> 
        }

        // pub fn get_masterchain_info(&mut self) -> Result<lite_result::MasterchainInfo> {
        //     self.lite_query(GetMasterchainInfo)
        // }

        // pub fn get_masterchain_info_ext(&mut self, mode: i32) -> Result<lite_result::MasterchainInfoExt> {
        //     self.lite_query(GetMasterchainInfoExt{mode})
        // }
        
        // pub fn get_time(&mut self) -> Result<lite_result::CurrentTime> {
        //     self.lite_query(GetTime)
        // }

        // pub fn get_version(&mut self) -> Result<lite_result::Version> {
        //     self.lite_query(GetVersion)
        // }

        // pub fn get_block(&mut self, id: BlockIdExt) -> Result<lite_result::BlockData> {
        //     self.lite_query(GetBlock{id})
        // }

        // pub fn get_state(&mut self, id: BlockIdExt) -> Result<lite_result::BlockState> {
        //     self.lite_query(GetState{id})
        // }

        // pub fn get_block_header(&mut self, id: BlockIdExt, mode: i32) -> Result<lite_result::BlockHeader> {
        //     self.lite_query(GetBlockHeader{id, mode})
        // }

        // pub fn send_message(&mut self, message: Vec<u8>) -> Result<lite_result::SendMsgStatus> {
        //     self.lite_query::<_, lite_result::SendMsgStatus>(SendMessage { body: bytes(message) })
        // }

        // pub fn get_account_state(&mut self, id: BlockIdExt, account: AccountId) -> Result<lite_result::AccountState> {
        //     self.lite_query(GetAccountState{id, account})
        // }

        // pub fn run_smc_method(&mut self, mode: i32, id: BlockIdExt, account: AccountId, method_id: i64, params: Vec<u8>) -> Result<lite_result::RunMethodResult> {
        //     self.lite_query(RunSmcMethod{mode, id, account, method_id, params: bytes(params)})
        // }

        // pub fn get_shard_info(&mut self, id: BlockIdExt, workchain: i32, shard: i64, exact: bool) -> Result<lite_result::ShardInfo> {
        //     let exact = if exact {Bool::BoolTrue} else {Bool::BoolFalse};
        //     self.lite_query(GetShardInfo{id, workchain, shard, exact})
        // }

        // pub fn get_all_shards_info(&mut self, id: BlockIdExt) -> Result<lite_result::AllShardsInfo> {
        //     self.lite_query(GetAllShardsInfo{id})
        // }

        // pub fn get_one_transaction(&mut self, id: BlockIdExt, account: AccountId, lt:i64) -> Result<lite_result::TransactionInfo> {
        //     self.lite_query(GetOneTransaction{id, account, lt})
        // }

        // pub fn get_transactions(&mut self, count: i32, account: AccountId, lt:i64, hash: [u8; 32]) -> Result<lite_result::TransactionList> {
        //     self.lite_query(GetTransactions{count, account, lt, hash: UInt256::with_array(hash)})
        // }

        // pub fn lookup_block(&mut self, mode: i32, id: BlockId, lt: Option<i64>, utime: Option<i32>) -> Result<lite_result::BlockHeader> {
        //     self.lite_query(LookupBlock{mode, id, lt, utime})
        // }

        // pub fn list_block_transactions(&mut self, id: BlockIdExt, mode: i32, count: i32, after: Option<TransactionId3>, reverse_order: bool, want_proof: bool) -> Result<lite_result::BlockTransactions> {
        //     self.lite_query(ListBlockTransactions{id, mode, count, after, reverse_order, want_proof})
        // }

        // pub fn get_block_proof(&mut self, mode: i32, known_block: BlockIdExt, target_block: Option<BlockIdExt>) -> Result<lite_result::PartialBlockProof> {
        //     self.lite_query(GetBlockProof{mode, known_block, target_block})
        // }

        // pub fn get_config_all(&mut self, mode: i32, id: BlockIdExt) -> Result<lite_result::ConfigInfo> {
        //     self.lite_query(GetConfigAll{mode, id})
        // }

        // pub fn get_config_params(&mut self, mode: i32, id: BlockIdExt, param_list: ton_api::ton::vector<Bare, i32>) -> Result<lite_result::ConfigInfo> {
        //     self.lite_query(GetConfigParams{mode, id, param_list})
        // }

        // pub fn get_validator_stats(&mut self, mode: i32, id: BlockIdExt, limit: i32, start_after: Option<[u8; 32]>, modified_after: Option<i32>) -> Result<lite_result::ValidatorStats> {
        //     let start_after = if start_after.is_some() {Some(UInt256::with_array(start_after.unwrap()))} else {None};
        //     self.lite_query(GetValidatorStats{mode, id, limit, start_after, modified_after})
        // }
    }
}