use std::error::Error;

use tests::{adnl::Message, common::Int256, request::WrappedRequest};

use crate::tl::*;

use self::request::Request;

#[test]
fn test_time() -> Result<(), Box<dyn Error>> {
    let raw = hex::decode("7af98bb435263e6c95d6fecb497dfd0aa5f031e7d412986b5ce720496db512052e8f2d100cdf068c7904345aad16000000000000")?;
    let message = Message::Query {
        query_id: Int256::from_hex("35263e6c95d6fecb497dfd0aa5f031e7d412986b5ce720496db512052e8f2d10")?, 
        query: request::LiteQuery { 
            wrapped_request: WrappedRequest { 
                request: Request::GetTime,
                wait_masterchain_seqno: None, 
            } 
        }
    };
    let constructed = tl_proto::serialize(message.clone());
    assert_eq!(raw, constructed);
    let deserialized = tl_proto::deserialize::<Message>(raw.as_slice())?;
    assert_eq!(deserialized, message);
    Ok(())
}