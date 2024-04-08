use std::sync::Arc;

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

use net_reporter_api::api::http_overview_dashboard_filters::http_overview_dashboard_filters::HttpOverviewDashboardFiltersDTO;
use net_reporter_api::api::http_overview_dashboard_filters::http_overview_dashboard_filters_request::HttpOverviewDashboardFiltersRequestDTO;

use crate::query::filters::http_overview::response::http_overview_filters::HttpOverviewFiltersResponse;

use crate::query::requester::Requester;

use super::endpoints_requester::EndpointsRequester;
use super::http_request_methods_requester::HttpRequestMethodsRequester;
use super::http_response_codes_requester::HttpResponseCodesRequester;


#[derive(Default)]
pub struct HttpOverviewFiltersRequester {}

impl HttpOverviewFiltersRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_queries(
        connection_pool: Arc<Pool<Postgres>>,
        group_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<HttpOverviewFiltersResponse, Error> {
        let endpoints = EndpointsRequester::execute_query(
            connection_pool.clone(),
            group_id,
            start_date,
            end_date,
        ).await?;
        let http_request_methods = HttpRequestMethodsRequester::execute_query(
            connection_pool.clone(),
            group_id,
            start_date,
            end_date,
        ).await?;
        let http_response_codes = HttpResponseCodesRequester::execute_query(
            connection_pool.clone(),
            group_id,
            start_date,
            end_date,
        ).await?;

        Ok(HttpOverviewFiltersResponse::new(
            endpoints,
            http_request_methods,
            http_response_codes
        ))
    }
}

#[async_trait::async_trait]
impl Requester for HttpOverviewFiltersRequester {
    async fn request_enveloped_chart(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != self.get_requesting_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = HttpOverviewDashboardFiltersRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();

        let executed_query_response = Self::execute_queries(
            connection_pool,
            tenant_id,
            request_start_date,
            request_end_date,
        ).await?;

        let response: HttpOverviewFiltersResponse = executed_query_response;
        log::info!("Got response on request: {:?}", response);

        let dto_response: HttpOverviewDashboardFiltersDTO = response.into();

        Ok(Envelope::new(
            tenant_id,
            HttpOverviewDashboardFiltersDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        HttpOverviewDashboardFiltersRequestDTO::get_data_type()
    }
}