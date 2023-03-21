use liteclient::{LiteClient, tl_types::{Int256, BlockIdExt, AccountId}};

#[test]
fn get_masterchain_info_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    assert!(client.get_masterchain_info().is_ok());
}

#[test]
fn get_masterchain_info_ext_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    assert!(client.get_masterchain_info_ext(0).is_ok());
}

#[test]
fn get_time_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    assert!(client.get_time().is_ok());
}

#[test]
fn get_version_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    assert!(client.get_version().is_ok());
}

#[test]
fn get_block_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let info = client.get_masterchain_info().unwrap();
    assert!(client.get_block(liteclient::tl_types::BlockIdExt{workchain: info.last.workchain, shard: info.last.shard, seqno: info.last.seqno, root_hash: info.last.root_hash, file_hash: info.last.file_hash}).is_ok());
}

#[test]
fn get_state_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let info = client.get_masterchain_info().unwrap();
    assert!(client.get_state(liteclient::tl_types::BlockIdExt{workchain: info.last.workchain, shard: info.last.shard, seqno: info.last.seqno, root_hash: info.last.root_hash, file_hash: info.last.file_hash}).is_ok());
}

#[test]
fn get_block_header() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let info = client.get_masterchain_info().unwrap();

    assert!(client.get_block(liteclient::tl_types::BlockIdExt{workchain: info.last.workchain, shard: info.last.shard, seqno: info.last.seqno, root_hash: info.last.root_hash, file_hash: info.last.file_hash}).is_ok());
}

#[test]
fn send_message_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    assert!(client.get_masterchain_info().is_ok());
}

#[test]
fn get_account_state_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let workchain: i32 = -1;
    let shard: u64 = 9223372036854775808;
    let seqno: u32 = 28235435;
    let root_hash = Int256::from_base64("Ah9jCut6khUq0CRBGdyjXkevGfX3VgtwP9/o4LVU4Io=").unwrap();
    let file_hash = Int256::from_base64("F7F/vvTky8+qofgh8fLkfNXe0C8ne1rR13YDxtfHYPI=").unwrap();
    let account = AccountId::from_friendly("EQA-PqYkjSr-bbu_dtpV379hZNiFXFGQGlr74SUOdOgSgxE0").unwrap();
    let res = client.get_account_state(BlockIdExt{workchain, shard, seqno, root_hash: root_hash.clone(), file_hash: file_hash.clone()}, account);
    assert!(res.is_ok());
    assert_eq!(res.unwrap().state, Vec::<u8>::new());
}

#[test]
fn run_smc_method_test() {
    todo!()
}

#[test]
fn get_shard_info_test() {
    todo!()
}

#[test]
fn get_all_shards_info_test() {
    todo!()
}

#[test]
fn get_one_transaction_test() {
    todo!()
}

#[test]
fn get_transactions_test() {
    todo!()
}

#[test]
fn lookup_block_test() {
    todo!()
}

#[test]
fn list_block_transactions_test() {
    todo!()
}

#[test]
fn get_block_proof_test() {
    todo!()
}

#[test]
fn get_config_all_test() {
    todo!()
}

#[test]
fn get_config_params_test() {
    todo!()
}

#[test]
fn get_validator_stats_test() {
    todo!()
}

