use std::sync::Arc;

use net_core_api::envelope::envelope::Envelope;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashboard_filters_request::NetworkOverviewDashboardFiltersRequestDTO;

use crate::query::charts::network_bandwidth::response::network_bandwidth::NetworkBandwidthResponse;
use crate::query::charts::network_bandwidth::response::bandwidth_bucket::BandwidthBucketResponse;
use crate::query::requester::Requester;

const NETWORK_OVERVIEW_FILTERS_QUERY: &str = "define postgres query here";

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
    ) -> Result<Vec<BandwidthBucketResponse>, Error> {
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

        let response: NetworkBandwidthResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkBandwidthDTO = response.into();

        Ok(Envelope::new(
            request_group_id,
            request_agent_id,
            NetworkBandwidthDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthRequestDTO::get_data_type()
    }
}