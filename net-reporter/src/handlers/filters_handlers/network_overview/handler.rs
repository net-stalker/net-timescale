use std::sync::Arc;

use net_component::handler::network_service_handler::NetworkServiceHandler;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;

use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashboard_filters_request::NetworkOverviewDashboardFiltersRequestDTO;

use crate::handlers::filters_handlers::network_overview::response::filter_entry::FilterEntryResponse;
use crate::handlers::filters_handlers::network_overview::response::network_overview_filters::NetworkOverviewFiltersResponse;

const NETWORK_OVERVIEW_FILTERS_QUERY: &str = "
SELECT
    COALESCE(lhs.IP, rhs.IP) AS Endpoint,
    ARRAY_REMOVE(ARRAY(SELECT DISTINCT unnest(string_to_array(COALESCE(lhs.Concatenated_Protocols, '') || ':' || COALESCE(rhs.Concatenated_Protocols, ''), ':'))), '') AS Protocols,
    GREATEST(lhs.Total_Bytes, rhs.Total_Bytes, 0) AS Total_Bytes
FROM
    (
        SELECT
            Src_IP AS IP,
            SUM(Packet_Length) AS Total_Bytes,
            STRING_AGG(Protocols, ':' ORDER BY Protocols) AS Concatenated_Protocols
        FROM Network_Overview_Filters_Materialized_View
        WHERE Tenant_ID = $1 AND Frametime >= $2 AND Frametime < $3
        GROUP BY Src_IP
    ) AS lhs FULL OUTER JOIN (
        SELECT
            Dst_IP AS IP,
            SUM(Packet_Length) AS Total_Bytes,
            STRING_AGG(Protocols, ':' ORDER BY Protocols) AS Concatenated_Protocols
        FROM Network_Overview_Filters_Materialized_View
        WHERE Tenant_ID = $1 AND Frametime >= $2 AND Frametime < $3
        GROUP BY Dst_IP
    ) AS rhs ON lhs.IP = rhs.IP;
";

#[derive(Default)]
pub struct NetworkOverviewFiltersHandler {}

impl NetworkOverviewFiltersHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<FilterEntryResponse>, Error> {
        sqlx::query_as(NETWORK_OVERVIEW_FILTERS_QUERY)
            .bind(tenant_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for NetworkOverviewFiltersHandler {
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

        let executed_query_response = Self::execute_query(
            connection_pool,
            tenant_id,
            request_start_date,
            request_end_date
        ).await?;

        let response: NetworkOverviewFiltersResponse = executed_query_response.into();
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