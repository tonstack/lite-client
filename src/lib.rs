pub use private::LiteClient;
pub use private::Result;

mod private {
    use std::error::Error;
    use adnl::client::{AdnlClient, AdnlClientConfig};
    use adnl::common::{serialize, TaggedTlObject};
    use ton_api::AnyBoxedSerialize;
    use ton_api::ton::rpc::lite_server;
    use ton_api::ton::{bytes, TLObject};
    use extfmt::AsHexdump;
    use ton_api::ton::lite_server::{CurrentTime, SendMsgStatus};
    use ton_api::ton::rpc::lite_server::{GetTime, SendMessage};

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
                object: TLObject::new(lite_server::Query {
                    data: bytes(serialize(&TLObject::new(request))?)
                })
            };
            log::debug!("Sending query {}", query.as_hexdump());
            let result = self.client.query(&query).await?;
            log::debug!("Received {}", result.as_hexdump());
            result.downcast::<U>().map_err(|_| "Downcast failed".into())
        }

        pub async fn get_time(&mut self) -> Result<CurrentTime> {
            self.lite_query(GetTime).await
        }

        pub async fn send_external_message(&mut self, message: Vec<u8>) -> Result<SendMsgStatus> {
            self.lite_query(SendMessage { body: bytes(message) }).await
        }
    }
}