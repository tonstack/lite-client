use hex::FromHex;
use liteclient::{tl_types::*, LiteClient};

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
    assert!(client
        .get_block(liteclient::tl_types::BlockIdExt {
            workchain: info.last.workchain,
            shard: info.last.shard,
            seqno: info.last.seqno,
            root_hash: info.last.root_hash,
            file_hash: info.last.file_hash
        })
        .is_ok());
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
    let workchain: i32 = -1;
    let shard: u64 = 9223372036854775808;
    let seqno: u32 = 2;
    let root_hash =
        liteclient::tl_types::Int256::from_base64("4bzgnFItQjTVEMYL9c/VHshMJttG9gDIXCzsMQdjKSU=")
            .unwrap();
    let file_hash =
        liteclient::tl_types::Int256::from_base64("2gOSTo8fuMWgA18snVD1RUtTfpU5LvCQWOOQ16Z7w5Y=")
            .unwrap();
    let result = (client).get_state(liteclient::tl_types::BlockIdExt {
        workchain,
        shard,
        seqno,
        root_hash,
        file_hash,
    });
    if result.is_err() {
        assert!(
            result.clone().unwrap_err().message
                == liteclient::tl_types::String::new("state not in db".to_string())
                || result.unwrap_err().message
                    == liteclient::tl_types::String::new("state already gc'd".to_string())
        );
    } else {
        assert!(!result.unwrap().data.is_empty());
    }
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

    assert!(client
        .get_block(liteclient::tl_types::BlockIdExt {
            workchain: info.last.workchain,
            shard: info.last.shard,
            seqno: info.last.seqno,
            root_hash: info.last.root_hash,
            file_hash: info.last.file_hash
        })
        .is_ok());
}

// #[test]
// fn send_message_test() {
//     // let config = {
//     //     let url = "https://ton-blockchain.github.io/global.config.json";
//     //     let mut response = ureq::get(url).call().unwrap();
//     //     while response.status() != 200 {
//     //         response = ureq::get(url).call().unwrap();
//     //     }
//     //     response.into_string().unwrap()
//     // };
//     // let mut client = LiteClient::connect(&config).unwrap();
//     todo!()
// }

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
    let account =
        AccountId::from_friendly("EQA-PqYkjSr-bbu_dtpV379hZNiFXFGQGlr74SUOdOgSgxE0").unwrap();
    let res = client
        .get_account_state(
            BlockIdExt {
                workchain,
                shard,
                seqno,
                root_hash,
                file_hash,
            },
            account,
        )
        .unwrap();
    assert!(res.state.is_empty());
}

#[test]
fn get_shard_info_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let block = (client)
        .lookup_block(
            BlockId {
                workchain: -1,
                shard: 9223372036854775808,
                seqno: 13000000,
            },
            None,
            None,
        )
        .unwrap();
    let result = (client)
        .get_shard_info(block.id, 0, 9223372036854775808, true)
        .unwrap();
    assert_eq!(
        result.shard_proof,
        vec![
            181u8, 238, 156, 114, 1, 2, 25, 2, 0, 4, 185, 1, 0, 9, 70, 3, 113, 120, 153, 245, 144,
            200, 82, 137, 148, 237, 70, 139, 171, 4, 196, 7, 81, 61, 189, 69, 103, 35, 28, 240,
            211, 231, 34, 135, 57, 26, 96, 254, 0, 43, 2, 9, 70, 3, 60, 23, 63, 231, 104, 223, 13,
            221, 50, 5, 48, 57, 218, 60, 98, 182, 83, 5, 112, 58, 67, 207, 94, 119, 27, 38, 237,
            107, 123, 57, 183, 162, 0, 20, 17, 36, 91, 144, 35, 175, 226, 255, 255, 255, 17, 0,
            255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 198, 93, 64, 0, 0, 0, 1, 96, 235, 12,
            87, 0, 0, 17, 190, 9, 84, 162, 132, 0, 198, 93, 61, 96, 3, 4, 5, 6, 40, 72, 1, 1, 185,
            20, 165, 199, 176, 95, 46, 123, 32, 111, 23, 172, 94, 35, 14, 216, 134, 60, 166, 34,
            226, 219, 72, 84, 179, 219, 160, 104, 149, 233, 28, 22, 0, 1, 40, 72, 1, 1, 61, 95, 80,
            117, 172, 122, 83, 138, 215, 75, 36, 31, 22, 245, 83, 155, 201, 227, 128, 154, 15, 114,
            238, 136, 76, 236, 170, 185, 107, 160, 102, 107, 0, 42, 34, 51, 0, 0, 0, 0, 0, 0, 0, 0,
            255, 255, 255, 255, 255, 255, 255, 255, 131, 147, 177, 167, 247, 26, 162, 90, 88, 40,
            7, 8, 36, 85, 204, 38, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
            170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170, 170,
            170, 170, 194, 47, 129, 187, 182, 96, 130, 3, 126, 9, 10, 11, 12, 40, 72, 1, 1, 165,
            167, 210, 64, 87, 216, 100, 59, 37, 39, 112, 157, 152, 108, 218, 56, 70, 173, 203, 62,
            221, 195, 45, 40, 236, 33, 246, 158, 23, 219, 170, 239, 0, 1, 40, 72, 1, 1, 120, 23,
            94, 12, 238, 3, 241, 51, 225, 183, 241, 225, 92, 134, 126, 183, 118, 155, 221, 83, 156,
            78, 66, 181, 127, 10, 143, 179, 161, 169, 177, 29, 0, 3, 1, 3, 208, 64, 13, 40, 72, 1,
            1, 102, 54, 213, 164, 15, 188, 211, 3, 125, 79, 255, 11, 179, 46, 82, 91, 90, 76, 209,
            226, 242, 225, 242, 204, 196, 91, 39, 202, 150, 4, 227, 74, 0, 15, 34, 191, 0, 1, 140,
            83, 216, 196, 0, 3, 24, 235, 96, 0, 2, 55, 193, 40, 172, 8, 136, 0, 0, 141, 219, 10,
            226, 244, 32, 6, 49, 207, 158, 208, 23, 94, 191, 143, 52, 74, 175, 118, 95, 237, 134,
            91, 248, 54, 154, 73, 45, 58, 6, 41, 251, 42, 235, 207, 99, 178, 87, 67, 112, 93, 235,
            63, 8, 52, 94, 254, 93, 234, 112, 101, 201, 181, 177, 67, 152, 17, 186, 153, 212, 245,
            196, 11, 58, 125, 2, 185, 198, 179, 206, 142, 146, 176, 56, 190, 15, 16, 40, 72, 1, 1,
            178, 14, 54, 163, 179, 106, 76, 222, 230, 1, 16, 108, 100, 46, 144, 113, 139, 10, 88,
            218, 242, 0, 117, 61, 187, 49, 137, 249, 86, 180, 148, 182, 0, 1, 1, 219, 80, 8, 88,
            98, 56, 6, 50, 234, 0, 0, 0, 141, 240, 74, 43, 2, 0, 0, 0, 141, 240, 74, 43, 2, 9, 94,
            99, 49, 148, 170, 1, 54, 197, 146, 50, 130, 198, 181, 213, 246, 201, 235, 187, 98, 24,
            202, 218, 203, 69, 28, 34, 0, 245, 147, 7, 4, 127, 46, 57, 13, 151, 80, 52, 157, 68,
            118, 94, 31, 49, 44, 31, 172, 54, 78, 129, 95, 33, 108, 209, 214, 15, 201, 249, 186,
            94, 36, 87, 182, 168, 128, 0, 24, 205, 252, 0, 0, 0, 0, 0, 0, 0, 0, 6, 50, 233, 235, 7,
            88, 98, 138, 14, 0, 19, 67, 185, 172, 160, 2, 29, 205, 101, 0, 32, 40, 72, 1, 1, 56,
            37, 159, 58, 45, 169, 231, 109, 91, 89, 182, 65, 62, 92, 224, 12, 116, 237, 34, 113, 2,
            228, 113, 99, 28, 145, 72, 223, 24, 100, 34, 134, 0, 24, 40, 72, 1, 1, 195, 104, 103,
            158, 204, 74, 140, 66, 116, 125, 48, 151, 21, 226, 230, 14, 208, 225, 137, 214, 252,
            50, 136, 58, 50, 58, 216, 154, 120, 227, 64, 82, 0, 15, 36, 16, 17, 239, 85, 170, 255,
            255, 255, 17, 18, 19, 20, 21, 1, 160, 155, 199, 169, 135, 0, 0, 0, 0, 4, 1, 0, 198, 93,
            64, 0, 0, 0, 1, 0, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 96, 235, 12, 87, 0, 0,
            17, 190, 9, 84, 162, 128, 0, 0, 17, 190, 9, 84, 162, 132, 85, 27, 118, 79, 0, 3, 24,
            235, 0, 198, 93, 61, 0, 198, 57, 243, 196, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 46, 22, 40,
            72, 1, 1, 191, 136, 21, 179, 231, 153, 234, 154, 206, 234, 51, 178, 128, 159, 84, 1,
            112, 140, 124, 135, 195, 250, 197, 122, 185, 124, 188, 240, 160, 177, 184, 51, 0, 3,
            42, 138, 4, 45, 71, 111, 148, 239, 45, 235, 46, 110, 102, 239, 239, 49, 26, 7, 103,
            168, 196, 60, 39, 209, 126, 181, 206, 121, 21, 22, 37, 111, 17, 50, 128, 113, 120, 153,
            245, 144, 200, 82, 137, 148, 237, 70, 139, 171, 4, 196, 7, 81, 61, 189, 69, 103, 35,
            28, 240, 211, 231, 34, 135, 57, 26, 96, 254, 0, 43, 0, 43, 23, 24, 40, 72, 1, 1, 11,
            227, 28, 51, 69, 166, 186, 82, 137, 76, 40, 146, 104, 138, 88, 1, 248, 181, 43, 182,
            115, 92, 66, 154, 234, 182, 243, 134, 104, 120, 33, 154, 0, 8, 0, 152, 0, 0, 17, 190,
            9, 69, 96, 68, 0, 198, 93, 63, 30, 244, 236, 125, 1, 234, 44, 173, 54, 186, 10, 225,
            243, 202, 242, 249, 236, 62, 93, 106, 250, 92, 231, 97, 3, 129, 48, 228, 234, 38, 165,
            160, 206, 30, 126, 251, 108, 53, 18, 141, 125, 76, 78, 224, 255, 41, 3, 17, 205, 125,
            145, 151, 27, 223, 227, 42, 220, 246, 195, 185, 165, 17, 100, 149, 104, 140, 1, 3, 45,
            71, 111, 148, 239, 45, 235, 46, 110, 102, 239, 239, 49, 26, 7, 103, 168, 196, 60, 39,
            209, 126, 181, 206, 121, 21, 22, 37, 111, 17, 50, 128, 254, 198, 24, 212, 236, 165, 51,
            249, 129, 79, 229, 120, 50, 253, 45, 122, 201, 164, 126, 204, 151, 71, 39, 42, 93, 11,
            188, 28, 191, 83, 75, 90, 0, 43, 0, 18, 104, 140, 1, 3, 113, 120, 153, 245, 144, 200,
            82, 137, 148, 237, 70, 139, 171, 4, 196, 7, 81, 61, 189, 69, 103, 35, 28, 240, 211,
            231, 34, 135, 57, 26, 96, 254, 208, 214, 128, 118, 188, 203, 190, 146, 211, 175, 18,
            82, 177, 18, 216, 16, 121, 121, 247, 209, 165, 240, 190, 146, 123, 114, 243, 148, 114,
            212, 204, 166, 0, 43, 0, 18
        ]
    );
}

#[test]
fn get_all_shards_info_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let block = (client)
        .lookup_block(
            BlockId {
                workchain: -1,
                shard: 9223372036854775808,
                seqno: 28330758,
            },
            None,
            None,
        )
        .unwrap();
    let result = (client).get_all_shards_info(block.id);
    assert!(result.is_ok());
}

#[test]
fn get_one_transaction_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let id = BlockIdExt {
        workchain: 0,
        seqno: 34073780,
        shard: 9223372036854775808,
        root_hash: Int256::from_base64("6JhaCv30PM3S4H5Zve/OMPHXUWv/LY0hOd9+M41VUi4=").unwrap(),
        file_hash: Int256::from_base64("2dL2E0uqZ5uDlKcPM11GeIPyaQgA4Hy7Zv3j2bKBhoI=").unwrap(),
    };
    let account =
        AccountId::from_friendly("EQAns9gBUsFtRnoOlwUuXW0738ol8CO5l46PlftBpzhRxmCs").unwrap();
    let result = client.get_one_transaction(id, account, 36479378000003);
    assert!(result.is_ok());
}

#[test]
fn get_transactions_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    // let id = BlockIdExt{workchain: 0, seqno: 34073780, shard: 9223372036854775808, root_hash: Int256::from_base64("6JhaCv30PM3S4H5Zve/OMPHXUWv/LY0hOd9+M41VUi4=").unwrap(), file_hash: Int256::from_base64("2dL2E0uqZ5uDlKcPM11GeIPyaQgA4Hy7Zv3j2bKBhoI=").unwrap()};
    let account =
        AccountId::from_friendly("EQAns9gBUsFtRnoOlwUuXW0738ol8CO5l46PlftBpzhRxmCs").unwrap();
    let result = client.get_transactions(
        10,
        account,
        36479378000003,
        Int256::from_base64("2JoweA1e8Lz4hDO1KZd1tyTYXsIXWv3FbkWeRL+PvsM=").unwrap(),
    );
    assert!(result.is_ok());
}

#[test]
fn lookup_block_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let block = liteclient::tl_types::BlockId {
        workchain: -1,
        shard: 3,
        seqno: 28327268,
    };
    let utime = 1679764594;
    let lt = 36333154000001;
    let empty_block = liteclient::tl_types::BlockId {
        workchain: -1,
        shard: 1,
        seqno: 1,
    };
    let res1 = client.lookup_block(block, None, None).unwrap();
    let res2 = client
        .lookup_block(empty_block.clone(), Some(lt), None)
        .unwrap();
    let res3 = client
        .lookup_block(empty_block, None, Some(utime))
        .unwrap();
    assert_eq!(res1, res2);
    assert_eq!(res1, res3);
}

#[test]
fn list_block_transactions_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let id = BlockIdExt {
        workchain: -1,
        seqno: 28455017,
        shard: 9223372036854775808,
        root_hash: Int256::from_base64("Re9R5ML5bOjRDb1H8/hDZIcVDZ6UuhzIlL38J9omfes=").unwrap(),
        file_hash: Int256::from_base64("kjP+8ReqSSiB6a6KKh9NKr/lov3RhFNZm0e/6Lndguc=").unwrap(),
    };
    let result = client.list_block_transactions(id, 10, None, None, None);
    assert!(result.is_ok());
}

#[test]
fn get_block_proof_test() {
    let hexfile = std::fs::read("tests/sample.txt").unwrap();
    let strin = Vec::<u8>::from_hex(hexfile).unwrap();
    let result = tl_proto::deserialize::<Message>(&strin).unwrap();
    let response: Vec<u8>;
    if let Message::Answer {
        query_id: _,
        answer,
    } = result
    {
        response = answer;
    } else {
        panic!();
    }
    let result2 = tl_proto::deserialize::<PartialBlockProof>(&response);
    assert!(result2.is_ok());
}

#[test]
fn get_config_all_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let id = BlockIdExt {
        workchain: -1,
        seqno: 28455017,
        shard: 9223372036854775808,
        root_hash: Int256::from_base64("Re9R5ML5bOjRDb1H8/hDZIcVDZ6UuhzIlL38J9omfes=").unwrap(),
        file_hash: Int256::from_base64("kjP+8ReqSSiB6a6KKh9NKr/lov3RhFNZm0e/6Lndguc=").unwrap(),
    };
    let result = client.get_config_all(0, id);
    assert!(result.is_ok());
}

#[test]
fn get_config_params_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let id = BlockIdExt {
        workchain: -1,
        seqno: 28455017,
        shard: 9223372036854775808,
        root_hash: Int256::from_base64("Re9R5ML5bOjRDb1H8/hDZIcVDZ6UuhzIlL38J9omfes=").unwrap(),
        file_hash: Int256::from_base64("kjP+8ReqSSiB6a6KKh9NKr/lov3RhFNZm0e/6Lndguc=").unwrap(),
    };
    let param_list = [0, 1, 2];
    let result = client.get_config_params(0, id, param_list.to_vec());
    assert!(result.is_ok());
}

#[test]
fn get_validator_stats_test() {
    let config = {
        let url = "https://ton-blockchain.github.io/global.config.json";
        let mut response = ureq::get(url).call().unwrap();
        while response.status() != 200 {
            response = ureq::get(url).call().unwrap();
        }
        response.into_string().unwrap()
    };
    let mut client = LiteClient::connect(&config).unwrap();
    let id = BlockIdExt {
        workchain: -1,
        seqno: 28455017,
        shard: 9223372036854775808,
        root_hash: Int256::from_base64("Re9R5ML5bOjRDb1H8/hDZIcVDZ6UuhzIlL38J9omfes=").unwrap(),
        file_hash: Int256::from_base64("kjP+8ReqSSiB6a6KKh9NKr/lov3RhFNZm0e/6Lndguc=").unwrap(),
    };
    let result = client.get_validator_stats(
        id.clone(),
        10,
        Some(Int256::from_base64("Re9R5ML5bOjRDb1H8/hDZIcVDZ6UuhzIlL38J9omfes=").unwrap()),
        None,
    );
    assert!(result.is_ok());
    let result = client.get_validator_stats(
        id.clone(),
        10,
        Some(Int256::from_base64("Re9R5ML5bOjRDb1H8/hDZIcVDZ6UuhzIlL38J9omfes=").unwrap()),
        Some(1650613847),
    );
    assert!(result.is_ok());
    let result = client.get_validator_stats(id, 10, None, None);
    assert!(result.is_ok());
}
