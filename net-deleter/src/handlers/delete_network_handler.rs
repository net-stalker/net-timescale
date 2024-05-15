use std::error::Error;
use std::sync::Arc;

use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;
use net_deleter_api::api::network::DeleteNetworkRequestDTO;
use sqlx::Pool;
use sqlx::Postgres;

#[derive(Default, Debug)]
pub struct DeleteNetworkHandler {}

impl DeleteNetworkHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for DeleteNetworkHandler {
    async fn handle(&self, connection_pool: Arc<Pool<Postgres>>, enveloped_request: Envelope) -> Result<Envelope, Box<dyn Error + Send + Sync>> {
        todo!()
    }

    fn get_handler_type(&self) -> String {
        DeleteNetworkRequestDTO::get_data_type().to_string()
    }
}
