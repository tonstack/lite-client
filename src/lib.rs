mod tests;

pub use private::LiteClient;
pub use private::Result;
pub use private::DeserializeError;

mod private {
    use std::error::Error;
    use ton_api::{AnyBoxedSerialize, deserialize_boxed, serialize_boxed};
    use ton_api::BoxedSerialize;
    use ton_api::ton::rpc::lite_server as lite_query;
    use ton_api::ton::{bytes, TLObject};
    use ton_api::ton::lite_server as lite_result;
    use ton_api::ton::adnl as adnl_tl;
    use ton_api::ton::rpc::lite_server::{GetTime, SendMessage};
    use pretty_hex::PrettyHex;
    use std::fmt::{Display, Formatter};
    use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
    use ton_types::UInt256;
    use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};
    use adnl::{AdnlAddress, AdnlClient, AdnlPublicKey, AdnlError, AdnlSecret, AdnlBuilder};
    use std::convert::TryInto;
    use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsPoint};
    use curve25519_dalek::montgomery::MontgomeryPoint;

    #[derive(Debug)]
    pub struct DeserializeError {
        object: TLObject,
    }

    impl Display for DeserializeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Deserialize error, can't downcast {:?}", self.object)
        }
    }

    impl Error for DeserializeError {}

    #[derive(Debug)]
    pub struct LiteError(lite_result::Error);

    impl Into<lite_result::Error> for LiteError {
        fn into(self) -> lite_result::Error {
            self.0
        }
    }

    impl From<lite_result::Error> for LiteError {
        fn from(e: lite_result::Error) -> Self {
            Self(e)
        }
    }

    impl Display for LiteError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Server error [code={}]: {}", self.0.code(), self.0.message())
        }
    }

    impl Error for LiteError {}

    pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

    pub fn to_montgomery(pub_key: &[u8; 32]) -> PublicKey {
        CompressedEdwardsY::from_slice(pub_key).decompress().unwrap().to_montgomery().to_bytes().into()
    }

    pub fn to_edwards(pub_key: &PublicKey) -> [u8; 32] {
        MontgomeryPoint(pub_key.to_bytes()).to_edwards(0).unwrap().compress().to_bytes()
    }

    pub struct LiteClient {
        client: AdnlClient<TcpStream>,
    }

    impl LiteClient {
        pub fn connect(config_json: &str) -> Result<Self> {
            let remote_public: [u8; 32] = base64::decode("JhXt7H1dZTgxQTIyGiYV4f9VUARuDxFl/1kVBjLSMB8=")?.try_into().unwrap();
            let remote_public_montgomery = to_montgomery(&remote_public);
            let local_secret = EphemeralSecret::new(rand::rngs::OsRng);
            let transport = TcpStream::connect(SocketAddrV4::new("65.21.74.140".parse()?, 46427))?;
            let client = AdnlBuilder::with_random_aes_params(&mut rand::rngs::OsRng)
                .use_static_ecdh(to_edwards(&PublicKey::from(&local_secret)),
                                 AdnlPublicKey::from(remote_public),
                                 local_secret.diffie_hellman(&remote_public_montgomery))
                .perform_handshake(transport).map_err(|e| format!("{:?}", e))?;
            Ok(Self { client })
        }

        pub fn lite_query<T: AnyBoxedSerialize, U: AnyBoxedSerialize>(&mut self, request: T) -> Result<U> {
            let mut message = serialize_boxed(&TLObject::new(adnl_tl::Message::Adnl_Message_Query(adnl_tl::message::message::Query {
                query_id: UInt256::with_array(rand::random()),
                query: bytes(serialize_boxed(
                    &TLObject::new(lite_query::Query {
                        data: bytes(serialize_boxed(&TLObject::new(request))?)
                    })
                )?),
            })))?;
            log::debug!("Sending query:\n{:?}", message.hex_dump());
            self.client.send(&mut message, &mut rand::random()).map_err(|e| format!("{:?}", e))?;
            let mut answer = Vec::<u8>::new();
            self.client.receive::<_, 8192>(&mut answer).map_err(|e| format!("{:?}", e))?;
            let result = deserialize_boxed(&answer)?.downcast::<adnl_tl::Message>().map_err(|o| Box::new(DeserializeError { object: o }))?;
            let result_obj = deserialize_boxed(&result.answer().unwrap().0)?;
            log::debug!("Received:\n{:?}", answer.hex_dump());
            result_obj.downcast::<U>().map_err(|e| {
                e.downcast::<lite_result::Error>().map(|e| Box::new(LiteError::from(e)).into())
                    .unwrap_or_else(|o| Box::new(DeserializeError { object: o }).into())
            })
        }

        pub fn get_time(&mut self) -> Result<lite_result::CurrentTime> {
            self.lite_query(GetTime)
        }

        pub fn send_external_message(&mut self, message: Vec<u8>) -> Result<lite_result::SendMsgStatus> {
            self.lite_query::<_, lite_result::SendMsgStatus>(SendMessage { body: bytes(message) })
        }
    }
}