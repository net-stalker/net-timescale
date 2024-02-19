use std::sync::Arc;

use net_core_api::decoder_api::Decoder;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashboard_filters_request::NetworkOverviewDashboardFiltersRequestDTO;

use crate::query::filters::network_overview::response::filter_entry::FilterEntryResponse;
use crate::query::filters::network_overview::response::network_overview_filters::NetworkOverviewFiltersResponse;
use crate::query::requester::Requester;

const NETWORK_OVERVIEW_FILTERS_QUERY: &str = "
        select
            COALESCE(lhs.id, rhs.id) as endpoint,
            ARRAY_REMOVE(ARRAY(SELECT DISTINCT unnest(string_to_array(COALESCE(lhs.concatenated_protocols, '') || ':' || COALESCE(rhs.concatenated_protocols, ''), ':'))), '') AS protocols,
            GREATEST(lhs.total_bytes, rhs.total_bytes, 0) as total_bytes
        from
            (
                select
                    src_addr as id,
                    SUM(packet_length) as total_bytes,
                    STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
                from network_overview_filters
                where group_id = $1 AND bucket >= $2 AND bucket < $3
                group by src_addr
            ) as lhs full outer join (
                select
                    dst_addr as id,
                    SUM(packet_length) as total_bytes,
                    STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
                from network_overview_filters
                where group_id = $1 AND bucket >= $2 AND bucket < $3
                group by dst_addr
            ) as rhs on lhs.id = rhs.id;
";

#[derive(Default)]
pub struct NetworkOverviewFiltersRequester {}

impl NetworkOverviewFiltersRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<FilterEntryResponse>, Error> {
        sqlx::query_as(NETWORK_OVERVIEW_FILTERS_QUERY)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await
    }
}

#[async_trait::async_trait]
impl Requester for NetworkOverviewFiltersRequester {
    async fn request(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope
    ) -> Result<Envelope, String> {
        let request_group_id = enveloped_request.get_group_id().ok();
        let request_agent_id = enveloped_request.get_agent_id().ok();

        if enveloped_request.get_type() != self.get_requesting_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()));
        }
        let request = NetworkOverviewDashboardFiltersRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();

        let executed_query_response = Self::execute_query(
            connection_pool,
            request_group_id,
            request_start_date,
            request_end_date
        ).await;

        if let Err(e) = executed_query_response {
            return Err(format!("error: {:?}", e));
        }
        let executed_query_response = executed_query_response.unwrap();

        let response: NetworkOverviewFiltersResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkOverviewDashboardFiltersDTO = response.into();

        Ok(Envelope::new(
            request_group_id,
            request_agent_id,
            NetworkOverviewDashboardFiltersDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkOverviewDashboardFiltersRequestDTO::get_data_type()
    }
}