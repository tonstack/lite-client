use derivative::Derivative;
use tl_proto::{TlRead, TlWrite};

use super::request::LiteQuery;
use super::response::Response;
use super::common::*;
use super::utils::*;

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(boxed)]
pub enum Message {
    /// adnl.message.query query_id:int256 query:bytes = adnl.Message;
    #[tl(id = 0xb48bf97a)]
    Query { query_id: Int256, #[tl(with = "struct_as_bytes")] query: LiteQuery },

    /// adnl.message.answer query_id:int256 answer:bytes = adnl.Message;
    #[tl(id = 0x0fac8416)]
    Answer { query_id: Int256, #[tl(with = "struct_as_bytes")] answer: Response },

    /// tcp.ping random_id:long = tcp.Pong;
    #[tl(id = 0x4d082b9a)]
    Ping { random_id: u64 },

    /// tcp.pong random_id:long = tcp.Pong;
    #[tl(id = 0xdc69fb03)]
    Pong { random_id: u64 },
}