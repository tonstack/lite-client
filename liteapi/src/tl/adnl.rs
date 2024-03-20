use derivative::Derivative;
use tl_proto::{TlRead, TlWrite};

use super::request::Query;
use super::response::Response;
use super::common::*;
use super::utils::*;

#[derive(TlRead, TlWrite, Derivative)]
#[derivative(Debug, Clone, PartialEq)]
#[tl(
    boxed,
    scheme_inline = r##"adnl.message.query query_id:int256 query:bytes = adnl.Message;
        adnl.message.answer query_id:int256 answer:bytes = adnl.Message;"##
)]
pub enum Message {
    /// adnl.message.query query_id:int256 query:bytes = adnl.Message;
    #[tl(id = "adnl.message.query")]
    Query { query_id: Int256, #[tl(with = "struct_as_bytes")] query: Query },

    /// adnl.message.answer query_id:int256 answer:bytes = adnl.Message;
    #[tl(id = "adnl.message.answer")]
    Answer { query_id: Int256, #[tl(with = "struct_as_bytes")] answer: Response },
}