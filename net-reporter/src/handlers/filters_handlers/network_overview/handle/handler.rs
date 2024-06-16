use std::sync::Arc;

use net_component::handler::network_service_handler::NetworkServiceHandler;
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashboard_filters_request::NetworkOverviewDashboardFiltersRequestDTO;
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use crate::handlers::filters_handlers::network_overview::response::network_overview_filters::NetworkOverviewFiltersResponse;
use crate::handlers::network_handlers::networks::handler::NetworksHandler;

use super::endpoints_handlers::EndpointsHandler;
use super::protocols_handler::ProtocolsHandler;


#[derive(Default)]
pub struct NetworksOverviewFiltersHandler {}

impl NetworksOverviewFiltersHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_queries(
        connection_pool: Arc<Pool<Postgres>>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<NetworkOverviewFiltersResponse, Error> {
        let mut transaction = connection_pool.begin().await?;
        let endpoints = EndpointsHandler::execute_query(
            &mut transaction,
            tenant_id,
            start_date,
            end_date,
        ).await?;

        let protocols = ProtocolsHandler::execute_query(
            &mut transaction,
            tenant_id,
            start_date,
            end_date,
        ).await?;

        let networks = NetworksHandler::execute_query(
            &mut transaction,
            &[],
            tenant_id
        ).await?;

        Ok(NetworkOverviewFiltersResponse::new(
            endpoints,
            protocols,
            networks,
        ))
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for NetworksOverviewFiltersHandler {
    async fn handle(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != self.get_handler_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = NetworkOverviewDashboardFiltersRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();

        let executed_query_response = Self::execute_queries(
            connection_pool,
            tenant_id,
            request_start_date,
            request_end_date,
        ).await?;

        let response: NetworkOverviewFiltersResponse = executed_query_response;
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkOverviewDashboardFiltersDTO = response.into();

        Ok(Envelope::new(
            tenant_id,
            NetworkOverviewDashboardFiltersDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_handler_type(&self) -> String {
        NetworkOverviewDashboardFiltersRequestDTO::get_data_type().to_string()
    }
}