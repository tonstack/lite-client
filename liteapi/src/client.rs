use adnl::AdnlPeer;
use tokio::net::ToSocketAddrs;
use tokio_tower::multiplex;
use tower::{Service as _, ServiceBuilder, ServiceExt as _};

use crate::{layers::{UnwrapErrorLayer, WrapMessagesLayer}, peer::LitePeer, tl::{common::*, request::*, response::*, utils::FromResponse}, types::LiteError};

type Result<T> = std::result::Result<T, LiteError>;

pub struct LiteClient {
    inner: tower::util::BoxService<
        WrappedRequest,
        Response,
        LiteError,
    >,
    wait_seqno: Option<u32>,
}

impl LiteClient {
    pub async fn connect<A: ToSocketAddrs>(address: A, public_key: impl AsRef<[u8]>) -> Result<Self> {
        let adnl = AdnlPeer::connect(public_key, address).await?;
        let lite = LitePeer::new(adnl);
        let service = ServiceBuilder::new()
            .layer(UnwrapErrorLayer)
            .layer(WrapMessagesLayer)
            .service(multiplex::Client::<_, Box<dyn std::error::Error + Send + Sync + 'static>, _>::new(lite));
        Ok(Self { inner: service.boxed(), wait_seqno: None })
    }

    pub fn wait_masterchain_seqno(mut self, seqno: u32) -> Self {
        self.wait_seqno = Some(seqno);
        self
    }

    async fn send_request<T: FromResponse>(&mut self, request: Request) -> Result<T>
    {
        let wrapped_request = WrappedRequest {
            wait_masterchain_seqno: self.wait_seqno.take().map(|seqno| WaitMasterchainSeqno { seqno, timeout_ms: 10000 }),
            request: request.into(),
        };
        T::from_response(self.inner.ready().await?.call(wrapped_request).await?)
    }

    pub async fn get_masterchain_info(&mut self) -> Result<MasterchainInfo> {
        let response: MasterchainInfo = self.send_request(Request::GetMasterchainInfo).await?;
        Ok(response)
    }

    pub async fn get_masterchain_info_ext(&mut self, mode: u32) -> Result<MasterchainInfoExt> {
        let request = Request::GetMasterchainInfoExt(GetMasterchainInfoExt { mode });
        let response: MasterchainInfoExt = self.send_request(request).await?;
        Ok(response)
    }

    pub async fn get_time(&mut self) -> Result<u32> {
        let response: CurrentTime = self.send_request(Request::GetTime).await?;
        Ok(response.now)
    }

    pub async fn get_version(&mut self) -> Result<Version> {
        let response: Version = self.send_request(Request::GetVersion).await?;
        Ok(response)
    }

    pub async fn get_block(&mut self, id: BlockIdExt) -> Result<Vec<u8>> {
        let request = Request::GetBlock(GetBlock { id });
        let response: BlockData = self.send_request(request).await?;
        Ok(response.data)
    }

    pub async fn get_state(&mut self, id: BlockIdExt) -> Result<BlockState> {
        let request = Request::GetState(GetState { id });
        let response: BlockState = self.send_request(request).await?;
        Ok(response)
    }

pub async fn get_block_header(
    &mut self,
    id: BlockIdExt,
    with_state_update: bool,
    with_value_flow: bool,
    with_extra: bool,
    with_shard_hashes: bool,
    with_prev_blk_signatures: bool,
) -> Result<Vec<u8>> {
        let request = Request::GetBlockHeader(GetBlockHeader {
            id,
            mode: (),
            with_state_update: if with_state_update { Some(()) } else { None },
            with_value_flow: if with_value_flow { Some(()) } else { None },
            with_extra: if with_extra { Some(()) } else { None },
            with_shard_hashes: if with_shard_hashes { Some(()) } else { None },
            with_prev_blk_signatures: if with_prev_blk_signatures { Some(()) } else { None },
        });
        let response: BlockHeader = self.send_request(request).await?;
        Ok(response.header_proof)
    }

    pub async fn send_message(&mut self, body: Vec<u8>) -> Result<u32> {
        let request = Request::SendMessage(SendMessage { body });
        let response: SendMsgStatus = self.send_request(request).await?;
        Ok(response.status)
    }

    pub async fn get_account_state(&mut self, id: BlockIdExt, account: AccountId) -> Result<AccountState> {
        let request = Request::GetAccountState(GetAccountState { id, account });
        let response: AccountState = self.send_request(request).await?;
        Ok(response)
    }

    pub async fn run_smc_method(&mut self, mode: u32, id: BlockIdExt, account: AccountId, method_id: u64, params: Vec<u8>) -> Result<RunMethodResult> {
        let request = Request::RunSmcMethod(RunSmcMethod { mode, id, account, method_id, params });
        let response: RunMethodResult = self.send_request(request).await?;
        Ok(response)
    }

    pub async fn get_shard_info(&mut self, id: BlockIdExt, workchain: i32, shard: u64, exact: bool) -> Result<ShardInfo> {
        let request = Request::GetShardInfo(GetShardInfo { id, workchain, shard, exact });
        let response: ShardInfo = self.send_request(request).await?;
        Ok(response)
    }

    pub async fn get_all_shards_info(&mut self, id: BlockIdExt) -> Result<AllShardsInfo> {
        let request = Request::GetAllShardsInfo(GetAllShardsInfo { id });
        let response: AllShardsInfo = self.send_request(request).await?;
        Ok(response)
    }

    pub async fn get_one_transaction(&mut self, id: BlockIdExt, account: AccountId, lt: u64) -> Result<TransactionInfo> {
        let request = Request::GetOneTransaction(GetOneTransaction { id, account, lt });
        let response: TransactionInfo = self.send_request(request).await?;
        Ok(response)
    }

    pub async fn get_transactions(&mut self, count: u32, account: AccountId, lt: u64, hash: Int256) -> Result<TransactionList> {
        let request = Request::GetTransactions(GetTransactions { count, account, lt, hash });
        let response: TransactionList = self.send_request(request).await?;
        Ok(response)
    }

pub async fn lookup_block(
    &mut self,
    mode: (),
    id: BlockId,
    seqno: Option<()>,
    lt: Option<u64>,
    utime: Option<u32>,
    with_state_update: bool,
    with_value_flow: bool,
    with_extra: bool,
    with_shard_hashes: bool,
    with_prev_blk_signatures: bool,
) -> Result<BlockHeader> {
        let request = Request::LookupBlock(LookupBlock {
            mode,
            id,
            seqno,
            lt,
            utime,
        with_state_update: if with_state_update { Some(()) } else { None },
        with_value_flow: if with_value_flow { Some(()) } else { None },
        with_extra: if with_extra { Some(()) } else { None },
        with_shard_hashes: if with_shard_hashes { Some(()) } else { None },
        with_prev_blk_signatures: if with_prev_blk_signatures { Some(()) } else { None },
    });
        let response: BlockHeader = self.send_request(request).await?;
        Ok(response)
    }

pub async fn list_block_transactions(
    &mut self,
    id: BlockIdExt,
    count: u32,
    after: Option<TransactionId3>,
    reverse_order: bool,
    want_proof: bool
) -> Result<BlockTransactions> {
    let request = Request::ListBlockTransactions(ListBlockTransactions {
        id,
        mode: (),
        count,
        after,
        reverse_order: if reverse_order { Some(()) } else { None },
        want_proof: if want_proof { Some(()) } else { None },
    });
        let response: BlockTransactions = self.send_request(request).await?;
        Ok(response)
    }

pub async fn get_block_proof(
    &mut self,
    known_block: BlockIdExt,
    target_block: Option<BlockIdExt>,
    allow_weak_target: bool,
    base_block_from_request: bool
) -> Result<PartialBlockProof> {
        let request = Request::GetBlockProof(GetBlockProof {
            mode: (),
            known_block,
            target_block,
        allow_weak_target: if allow_weak_target { Some(()) } else { None },
        base_block_from_request: if base_block_from_request { Some(()) } else { None },
        });
        let response: PartialBlockProof = self.send_request(request).await?;
        Ok(response)
    }

pub async fn get_config_all(
    &mut self,
    id: BlockIdExt,
    with_state_root: bool,
    with_libraries: bool,
    with_state_extra_root: bool,
    with_shard_hashes: bool,
    with_validator_set: bool,
    with_special_smc: bool,
    with_accounts_root: bool,
    with_prev_blocks: bool,
    with_workchain_info: bool,
    with_capabilities: bool,
    extract_from_key_block: bool,
) -> Result<ConfigInfo> {
        let request = Request::GetConfigAll(GetConfigAll {
            mode: (),
            id,
            with_state_root: if with_state_root { Some(()) } else { None },
            with_libraries: if with_libraries { Some(()) } else { None },
            with_state_extra_root: if with_state_extra_root { Some(()) } else { None },
            with_shard_hashes: if with_shard_hashes { Some(()) } else { None },
            with_validator_set: if with_validator_set { Some(()) } else { None },
            with_special_smc: if with_special_smc { Some(()) } else { None },
            with_accounts_root: if with_accounts_root { Some(()) } else { None },
            with_prev_blocks: if with_prev_blocks { Some(()) } else { None },
            with_workchain_info: if with_workchain_info { Some(()) } else { None },
            with_capabilities: if with_capabilities { Some(()) } else { None },
            extract_from_key_block: if extract_from_key_block { Some(()) } else { None },
        });
        let response: ConfigInfo = self.send_request(request).await?;
        Ok(response)
    }

pub async fn get_config_params(
    &mut self,
    id: BlockIdExt,
    param_list: Vec<i32>,
    with_state_root: bool,
    with_libraries: bool,
    with_state_extra_root: bool,
    with_shard_hashes: bool,
    with_validator_set: bool,
    with_special_smc: bool,
    with_accounts_root: bool,
    with_prev_blocks: bool,
    with_workchain_info: bool,
    with_capabilities: bool,
    extract_from_key_block: bool,
) -> Result<ConfigInfo> {
        let request = Request::GetConfigParams(GetConfigParams {
            mode: (),
            id,
            param_list,
            with_state_root: if with_state_root { Some(()) } else { None },
            with_libraries: if with_libraries { Some(()) } else { None },
            with_state_extra_root: if with_state_extra_root { Some(()) } else { None },
            with_shard_hashes: if with_shard_hashes { Some(()) } else { None },
            with_validator_set: if with_validator_set { Some(()) } else { None },
            with_special_smc: if with_special_smc { Some(()) } else { None },
            with_accounts_root: if with_accounts_root { Some(()) } else { None },
            with_prev_blocks: if with_prev_blocks { Some(()) } else { None },
            with_workchain_info: if with_workchain_info { Some(()) } else { None },
            with_capabilities: if with_capabilities { Some(()) } else { None },
            extract_from_key_block: if extract_from_key_block { Some(()) } else { None },
        });
        let response: ConfigInfo = self.send_request(request).await?;
        Ok(response)
    }

    pub async fn get_validator_stats(&mut self, id: BlockIdExt, limit: u32, start_after: Option<Int256>, modified_after: Option<u32>) -> Result<ValidatorStats> {
        let request = Request::GetValidatorStats(GetValidatorStats { mode: (), id, limit, start_after, modified_after });
        let response: ValidatorStats = self.send_request(request).await?;
        Ok(response)
    }

    pub async fn get_libraries(&mut self, library_list: Vec<Int256>) -> Result<Vec<LibraryEntry>> {
        let request = Request::GetLibraries(GetLibraries { library_list });
        let response: LibraryResult = self.send_request(request).await?;
        Ok(response.result)
    }
}