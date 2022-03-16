mod adnl;

pub use private::LiteClient;
pub use private::Result;
pub use private::DeserializeError;

mod private {
    use std::error::Error;
    use adnl::client::{AdnlClient, AdnlClientConfig};
    use adnl::common::{serialize, TaggedTlObject};
    use ton_api::AnyBoxedSerialize;
    use ton_api::BoxedSerialize;
    use ton_api::ton::rpc::lite_server as lite_query;
    use ton_api::ton::{bytes, TLObject};
    use ton_api::ton::lite_server as lite_result;
    use ton_api::ton::rpc::lite_server::{GetTime, SendMessage};
    use pretty_hex::PrettyHex;
    use std::fmt::{Display, Formatter};

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

    pub struct LiteClient {
        client: AdnlClient,
    }

    impl LiteClient {
        pub async fn connect(config_json: &str) -> Result<Self> {
            let adnl_config = AdnlClientConfig::from_json(config_json)?.1;
            Ok(Self { client: AdnlClient::connect(&adnl_config).await? })
        }

        pub async fn lite_query<T: AnyBoxedSerialize, U: AnyBoxedSerialize>(&mut self, request: T) -> Result<U> {
            let query = TaggedTlObject {
                object: TLObject::new(lite_query::Query {
                    data: bytes(serialize(&TLObject::new(request))?)
                })
            };
            log::debug!("Sending query:\n{:?}", query.object.boxed_serialized_bytes()?.hex_dump());
            let result = self.client.query(&query).await?;
            log::debug!("Received:\n{:?}", result.boxed_serialized_bytes()?.hex_dump());
            result.downcast::<U>().map_err(|e| {
                e.downcast::<lite_result::Error>().map(|e| Box::new(LiteError::from(e)).into())
                    .unwrap_or_else(|o| Box::new(DeserializeError { object: o }).into())
            })
        }

        pub async fn get_time(&mut self) -> Result<lite_result::CurrentTime> {
            self.lite_query(GetTime).await
        }

        pub async fn send_external_message(&mut self, message: Vec<u8>) -> Result<lite_result::SendMsgStatus> {
            self.lite_query::<_, lite_result::SendMsgStatus>(SendMessage { body: bytes(message) }).await
        }
    }
}