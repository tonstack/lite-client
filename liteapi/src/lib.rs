pub mod tl_types;

pub use private::{LiteClient, LiteError};

mod private {
    use std::{
        error::Error,
        fmt::{Debug, Display},
    };

    use crate::tl_types;
    use adnl::{AdnlBuilder, AdnlClient, AdnlError, AdnlPublicKey, Empty};
    use ciborium_io::{Read, Write};
    use tl_proto::{TlError, TlRead, TlWrite};
    use x25519_dalek::StaticSecret;

    pub enum LiteError<T: Read + Write> {
        ServerError(tl_types::Error),
        TlError(TlError),
        NotLiteAnswer,
        AdnlReadError(<T as Read>::Error),
        AdnlWriteError(<T as Write>::Error),
        AdnlIntegrityError,
        AdnlTooShortPacketError,
    }

    impl<T: Read + Write> Display for LiteError<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl<T: Read + Write> Debug for LiteError<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::ServerError(arg0) => f.debug_tuple("ServerError").field(arg0).finish(),
                Self::TlError(arg0) => f.debug_tuple("TlError").field(arg0).finish(),
                Self::NotLiteAnswer => write!(f, "NotLiteAnswer"),
                Self::AdnlReadError(arg0) => write!(f, "AdnlReadError"),
                Self::AdnlWriteError(arg0) => write!(f, "AdnlWriteError"),
                Self::AdnlIntegrityError => write!(f, "AdnlIntegrityError"),
                Self::AdnlTooShortPacketError => write!(f, "AdnlTooShortPacketError"),
            }
        }
    }

    impl<T: Read + Write> Error for LiteError<T> {}

    pub type LiteResult<T, U> = std::result::Result<U, LiteError<T>>;

    pub struct LiteClient<T: Read + Write> {
        client: AdnlClient<T>,
    }

    impl<T: Read + Write> LiteClient<T> {
        pub fn connect<P: AdnlPublicKey>(
            transport: T,
            public_key: P,
        ) -> Result<Self, LiteError<T>> {
            let local_secret = StaticSecret::new(rand::rngs::OsRng);
            let client = AdnlBuilder::with_random_aes_params(&mut rand::rngs::OsRng)
                .perform_ecdh(local_secret, public_key)
                .perform_handshake(transport).map_err(|e| match e {
                    AdnlError::ReadError(e) => LiteError::AdnlReadError(e),
                    AdnlError::WriteError(e) => LiteError::AdnlWriteError(e),
                    AdnlError::ConsumeError(_) => unreachable!(),
                    AdnlError::IntegrityError => LiteError::AdnlIntegrityError,
                    AdnlError::TooShortPacket => LiteError::AdnlTooShortPacketError,
                })?;
            Ok(Self { client })
        }

        pub fn lite_query<'tl, R, U>(
            &mut self,
            request: R,
            response: &'tl mut Vec<u8>,
        ) -> LiteResult<T, U>
        where
            R: TlWrite + Debug,
            U: TlRead<'tl> + Debug,
        {
            log::debug!("liteapi request:\n{:#?}", request);

            // pack lite api query
            let mut message = tl_proto::serialize(tl_types::Message::Query {
                query_id: (tl_types::Int256(rand::random())),
                query: (tl_proto::serialize(tl_types::Query {
                    data: (tl_proto::serialize(request)),
                })),
            });

            // send over ADNL
            self.client
                .send(&mut message, &mut rand::random())
                .map_err(|e| match e {
                    AdnlError::WriteError(x) => LiteError::AdnlWriteError(x),
                    _ => unreachable!(),
                })?;

            // get response over ADNL
            self.client
                .receive::<_, 8192>(response)
                .map_err(|e| match e {
                    AdnlError::ReadError(e) => LiteError::AdnlReadError(e),
                    AdnlError::IntegrityError => LiteError::AdnlIntegrityError,
                    AdnlError::TooShortPacket => LiteError::AdnlTooShortPacketError,
                    _ => unreachable!(),
                })?;

            // deserialize adnl.Message
            let data = tl_proto::deserialize::<tl_types::Message>(response)
                .map_err(|e| LiteError::TlError(e))?;
            match data {
                tl_types::Message::Answer {
                    query_id: _,
                    answer,
                } => {
                    *response = answer;
                }
                msg => {
                    log::error!("Got wrong adnl.Message type from server, expected adnl.message.answer:\n{:#?}", msg);
                    return Err(LiteError::NotLiteAnswer);
                }
            }

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

            log::debug!("liteapi response ok:\n{:#?}", result);
            Ok(result)
        }

        pub fn get_masterchain_info(&mut self) -> LiteResult<T, tl_types::MasterchainInfo> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetMasterchainInfo, &mut response)
        }

        pub fn get_masterchain_info_ext(
            &mut self,
            mode: i32,
        ) -> LiteResult<T, tl_types::MasterchainInfoExt> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetMasterchainInfoExt { mode }, &mut response)
        }

        pub fn get_time(&mut self) -> LiteResult<T, tl_types::CurrentTime> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetTime, &mut response)
        }

        pub fn get_version(&mut self) -> LiteResult<T, tl_types::Version> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetVersion, &mut response)
        }

        pub fn get_block(
            &mut self,
            id: tl_types::BlockIdExt,
        ) -> LiteResult<T, tl_types::BlockData> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetBlock { id }, &mut response)
        }

        pub fn get_state(
            &mut self,
            id: tl_types::BlockIdExt,
        ) -> LiteResult<T, tl_types::BlockState> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetState { id }, &mut response)
        }

        pub fn get_block_header(
            &mut self,
            id: tl_types::BlockIdExt,
            mode: i32,
        ) -> LiteResult<T, tl_types::BlockHeader> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetBlockHeader { id, mode }, &mut response)
        }

        pub fn send_message(&mut self, body: Vec<u8>) -> LiteResult<T, tl_types::SendMsgStatus> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::SendMessage { body }, &mut response)
        }

        pub fn get_account_state(
            &mut self,
            id: tl_types::BlockIdExt,
            account: tl_types::AccountId,
        ) -> LiteResult<T, tl_types::AccountState> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetAccountState { id, account }, &mut response)
        }

        pub fn run_smc_method(
            &mut self,
            id: tl_types::BlockIdExt,
            account: tl_types::AccountId,
            method_id: i64,
            params: Vec<u8>,
        ) -> LiteResult<T, tl_types::RunMethodResult> {
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::RunSmcMethod {
                    mode: (),
                    id,
                    account,
                    method_id,
                    params,
                },
                &mut response,
            )
        }

        pub fn get_shard_info(
            &mut self,
            id: tl_types::BlockIdExt,
            workchain: i32,
            shard: u64,
            exact: bool,
        ) -> LiteResult<T, tl_types::ShardInfo> {
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::GetShardInfo {
                    id,
                    workchain,
                    shard,
                    exact,
                },
                &mut response,
            )
        }

        pub fn get_all_shards_info(
            &mut self,
            id: tl_types::BlockIdExt,
        ) -> LiteResult<T, tl_types::AllShardsInfo> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetAllShardsInfo { id }, &mut response)
        }

        pub fn get_one_transaction(
            &mut self,
            id: tl_types::BlockIdExt,
            account: tl_types::AccountId,
            lt: i64,
        ) -> LiteResult<T, tl_types::TransactionInfo> {
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::GetOneTransaction { id, account, lt },
                &mut response,
            )
        }

        pub fn get_transactions(
            &mut self,
            count: i32,
            account: tl_types::AccountId,
            lt: i64,
            hash: tl_types::Int256,
        ) -> LiteResult<T, tl_types::TransactionList> {
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::GetTransactions {
                    count,
                    account,
                    lt,
                    hash,
                },
                &mut response,
            )
        }

        pub fn lookup_block(
            &mut self,
            id: tl_types::BlockId,
            lt: Option<i64>,
            utime: Option<i32>,
        ) -> LiteResult<T, tl_types::BlockHeader> {
            let trash = if lt.is_none() && utime.is_none() {
                Some(0u8)
            } else {
                None
            };
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::LookupBlock {
                    mode: (),
                    trash,
                    id,
                    lt,
                    utime,
                },
                &mut response,
            )
        }

        pub fn list_block_transactions(
            &mut self,
            id: tl_types::BlockIdExt,
            count: i32,
            after: Option<tl_types::TransactionId3>,
            reverse_order: Option<tl_types::True>,
            want_proof: Option<tl_types::True>,
        ) -> LiteResult<T, tl_types::BlockTransactions> {
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::ListBlockTransactions {
                    id,
                    mode: (),
                    count,
                    after,
                    reverse_order,
                    want_proof,
                },
                &mut response,
            )
        }

        pub fn get_block_proof(
            &mut self,
            known_block: tl_types::BlockIdExt,
            target_block: Option<tl_types::BlockIdExt>,
        ) -> LiteResult<T, tl_types::PartialBlockProof> {
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::GetBlockProof {
                    mode: (),
                    known_block,
                    target_block,
                },
                &mut response,
            )
        }

        pub fn get_config_all(
            &mut self,
            mode: i32,
            id: tl_types::BlockIdExt,
        ) -> LiteResult<T, tl_types::ConfigInfo> {
            let mut response = Vec::<u8>::new();
            self.lite_query(tl_types::GetConfigAll { mode, id }, &mut response)
        }

        pub fn get_config_params(
            &mut self,
            mode: i32,
            id: tl_types::BlockIdExt,
            param_list: Vec<i32>,
        ) -> LiteResult<T, tl_types::ConfigInfo> {
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::GetConfigParams {
                    mode,
                    id,
                    param_list,
                },
                &mut response,
            )
        }

        pub fn get_validator_stats(
            &mut self,
            id: tl_types::BlockIdExt,
            limit: i32,
            start_after: Option<tl_types::Int256>,
            modified_after: Option<i32>,
        ) -> LiteResult<T, tl_types::ValidatorStats> {
            let mut response = Vec::<u8>::new();
            self.lite_query(
                tl_types::GetValidatorStats {
                    mode: (),
                    id,
                    limit,
                    start_after,
                    modified_after,
                },
                &mut response,
            )
        }
    }
}
