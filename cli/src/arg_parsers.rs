use std::error::Error;

use regex::Regex;
use ton_liteapi::tl::common::{Int256, BlockIdExt, AccountId};

pub fn parse_block_id_ext(s: &str) -> std::result::Result<BlockIdExt, String> {
    let re = Regex::new(r"\(([-]?\d+),([a-fA-F0-9]+),(\d+)\):([^:]+):(.+)").unwrap();

    if let Some(capture) = re.captures(s) {
        let workchain = capture[1]
            .parse::<i32>()
            .map_err(|e| format!("Can't parse workchain '{}': {:?}", &capture[1], e))?;
        let shard = u64::from_str_radix(&capture[2], 16)
            .map_err(|e| format!("Can't parse shard '{}': {:?}", &capture[2], e))?;
        let seqno = capture[3]
            .parse::<u32>()
            .map_err(|e| format!("Can't parse seqno '{}': {:?}", &capture[3], e))?;
        let root_hash = capture[4].parse::<Int256>()
            .map_err(|e| format!("Can't parse root_hash '{}': {:?}", &capture[4], e))?;
        let file_hash = capture[5].parse::<Int256>()
            .map_err(|e| format!("Can't parse file_hash '{}': {:?}", &capture[5], e))?;
        Ok(BlockIdExt {
            workchain,
            shard,
            seqno,
            root_hash,
            file_hash,
        })
    } else {
        Err("Wrong format, must be (workchain,shard_hex,seqno):root_hash:file_hash".into())
    }
}

fn parse_account_base64(s: &str) -> std::result::Result<AccountId, Box<dyn Error>> {
    let res = base64::decode_config(s, base64::URL_SAFE)?;
    if res.len() != 36 {
        return Err(format!("Wrong length for base64 address, expected 36, got {}", res.len()).into())
    }
    let workchain = res[1] as i8;
    let id = Int256((&res[2..34]).try_into()?);
    Ok(AccountId {
        workchain: workchain.into(),
        id,
    })
}

fn parse_account_raw(s: &str) -> std::result::Result<AccountId, Box<dyn Error>> {
    let (workchain, account) = s.split_once(":").ok_or_else(|| format!("can't parse {}: wrong address format, must be <workchain>:<account>", s))?;
    let workchain = workchain.parse::<i32>().map_err(|_e| format!("wrong workchain {}", workchain))?;
    let id = account.parse::<Int256>().map_err(|_e| format!("wrong account id {}", account))?;
    Ok(AccountId { workchain, id })
}

pub fn parse_account_id(s: &str) -> std::result::Result<AccountId, String> {
    parse_account_base64(s).or_else(|e| parse_account_raw(s).map_err(|e2| format!("Can't parse account as base64 ({}) or as raw ({}))", e, e2)))
}

pub fn parse_key(s: &str) -> std::result::Result<[u8; 32], Box<dyn Error + Send + Sync>> {
    Ok(base64::decode(s).or_else(|_e| hex::decode(s)).map_err(|_e| "can't parse key")?.as_slice().try_into()?)
}