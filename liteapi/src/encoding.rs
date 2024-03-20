use tl_proto::{TlError, TlWrite};
use tokio_util::{bytes::BytesMut, codec::{Decoder, Encoder}};

use crate::tl::adnl::Message;

pub struct LiteCodec;

impl Decoder for LiteCodec {
    type Item = Message;
    type Error = TlCodecError;
    
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() == 0 {
            return Ok(None)
        }
        Ok(Some(tl_proto::deserialize::<Message>(src).map_err(|e| TlCodecError::TlError(e))?))
    }
}

impl Encoder<Message> for LiteCodec {
    type Error = TlCodecError;
    
    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write_to(dst);
        Ok(())
    }
}

#[derive(Debug)]
pub enum TlCodecError {
    TlError(TlError),
    IoError(std::io::Error),
}

impl From<std::io::Error> for TlCodecError {
    fn from(value: std::io::Error) -> Self {
        TlCodecError::IoError(value)
    }
}