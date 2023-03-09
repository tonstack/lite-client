pub mod config;

pub mod tl_types;

pub use private::LiteClient;
pub use private::Result;

mod private {
    use std::error::Error;
    use pretty_hex::PrettyHex;
    use std::net::TcpStream;
    use x25519_dalek::StaticSecret;
    use adnl::{AdnlClient, AdnlBuilder};
    use rand::prelude::SliceRandom;
    use crate::config::ConfigGlobal;
    use crate::tl_types;
    use tl_proto::{TlWrite, TlResult, TlRead};

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
            U: TlRead<'tl> 
        {
            let mut message = tl_proto::serialize(tl_types::Message::Query { 
                query_id: (tl_types::Int256(rand::random())), 
                query: (tl_proto::serialize(tl_types::Query{data: (tl_proto::serialize(request))})) 
            });
            
            log::debug!("Sending query:\n{:?}", &message.hex_dump());
            self.client.send(&mut message, &mut rand::random())
                .map_err(|e| format!("{:?}", e)).unwrap();
            log::debug!("Query sent");
            self.client.receive::<_, 8192>(response)
                .map_err(|e| format!("{:?}", e)).unwrap();
            log::debug!("Received:\n{:?}", &response.hex_dump());
            let data = tl_proto::deserialize::<tl_types::Message>(response).unwrap();
            if let tl_types::Message::Answer { query_id: _, answer} = data {
                *response = answer;
            }
            else {panic!();}
            tl_proto::deserialize::<U>(response)
        }

        pub fn get_masterchain_info(&mut self) -> TlResult<tl_types::MasterchainInfo> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetMasterchainInfo, &mut response) as TlResult<tl_types::MasterchainInfo> 
        }

        pub fn get_masterchain_info_ext(&mut self, mode: i32) -> TlResult<tl_types::MasterchainInfoExt> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetMasterchainInfoExt{mode}, &mut response) as TlResult<tl_types::MasterchainInfoExt> 
        }
        
        pub fn get_time(&mut self) -> TlResult<tl_types::CurrentTime> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetTime, &mut response) as TlResult<tl_types::CurrentTime> 
        }

        pub fn get_version(&mut self) -> TlResult<tl_types::Version> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetVersion, &mut response) as TlResult<tl_types::Version> 
        }

        pub fn get_block(&mut self, id: tl_types::BlockIdExt) -> TlResult<tl_types::BlockData> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetBlock{id}, &mut response) as TlResult<tl_types::BlockData> 
        }
    
        pub fn get_state(&mut self, id: tl_types::BlockIdExt) -> TlResult<tl_types::BlockState> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetState{id}, &mut response) as TlResult<tl_types::BlockState> 
        }

        pub fn get_block_header(&mut self, id: tl_types::BlockIdExt, mode: ()) -> TlResult<tl_types::BlockHeader> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetBlockHeader{id, mode}, &mut response) as TlResult<tl_types::BlockHeader> 
        }

        pub fn send_message(&mut self, body: Vec<u8>) -> TlResult<tl_types::SendMsgStatus> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::SendMessage{body}, &mut response) as TlResult<tl_types::SendMsgStatus> 
        }

        pub fn get_account_state(&mut self, id: tl_types::BlockIdExt, account: tl_types::AccountId) -> TlResult<tl_types::AccountState> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetAccountState{id, account}, &mut response) as TlResult<tl_types::AccountState> 
        }

        pub fn run_smc_method(&mut self, id: tl_types::BlockIdExt, account: tl_types::AccountId, method_id: i64, params: Vec<u8>) -> TlResult<tl_types::RunMethodResult> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::RunSmcMethod{mode: (), id, account, method_id, params}, &mut response) as TlResult<tl_types::RunMethodResult> 
        }

        pub fn get_shard_info(&mut self, id: tl_types::BlockIdExt, workchain: i32, shard: i64, exact: bool) -> TlResult<tl_types::ShardInfo> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetShardInfo{id, workchain, shard, exact}, &mut response) as TlResult<tl_types::ShardInfo> 
        }

        pub fn get_all_shards_info(&mut self, id: tl_types::BlockIdExt) -> TlResult<tl_types::AllShardsInfo> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetAllShardsInfo{id}, &mut response) as TlResult<tl_types::AllShardsInfo> 
        }

        pub fn get_one_transaction(&mut self, id: tl_types::BlockIdExt, account: tl_types::AccountId, lt: i64) -> TlResult<tl_types::TransactionInfo> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetOneTransaction{id, account, lt}, &mut response) as TlResult<tl_types::TransactionInfo> 
        }

        pub fn get_transactions(&mut self, count: i32, account: tl_types::AccountId, lt:i64, hash: tl_types::Int256) -> TlResult<tl_types::TransactionList> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetTransactions{count, account, lt, hash}, &mut response) as TlResult<tl_types::TransactionList> 
        }

        pub fn lookup_block(&mut self, id: tl_types::BlockId, lt: Option<i64>, utime: Option<i32>) -> TlResult<tl_types::BlockHeader> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::LookupBlock{mode: (), id, lt, utime}, &mut response) as TlResult<tl_types::BlockHeader> 
        }

        pub fn list_block_transactions(&mut self, id: tl_types::BlockIdExt, count: i32, after: Option<tl_types::TransactionId3>, reverse_order: Option<tl_types::True>, want_proof: Option<tl_types::True>) -> TlResult<tl_types::BlockTransactions> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::ListBlockTransactions{id, mode: (), count, after, reverse_order, want_proof}, &mut response) as TlResult<tl_types::BlockTransactions> 
        }

        pub fn get_block_proof(&mut self, known_block: tl_types::BlockIdExt, target_block: Option<tl_types::BlockIdExt>) -> TlResult<tl_types::PartialBlockProof> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetBlockProof{mode: (), known_block, target_block}, &mut response) as TlResult<tl_types::PartialBlockProof> 
        }

        pub fn get_config_all(&mut self, id: tl_types::BlockIdExt) -> TlResult<tl_types::ConfigInfo> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetConfigAll{mode: (), id}, &mut response) as TlResult<tl_types::ConfigInfo> 
        }

        pub fn get_config_params(&mut self, id: tl_types::BlockIdExt, param_list: Vec<i32>) -> TlResult<tl_types::ConfigInfo> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetConfigParams{mode: (), id, param_list}, &mut response) as TlResult<tl_types::ConfigInfo> 
        }

        pub fn get_validator_stats(&mut self, id: tl_types::BlockIdExt, limit: i32, start_after: Option<tl_types::Int256>, modified_after: Option<i32>) -> TlResult<tl_types::ValidatorStats> {
            let  mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetValidatorStats{mode: (), id, limit, start_after, modified_after}, &mut response) as TlResult<tl_types::ValidatorStats> 
        }
    }
}
