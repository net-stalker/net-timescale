use std::sync::Arc;

use net_token_verifier::fusion_auth::jwt_token::Jwt;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::envelope::envelope::Envelope;
use net_core_api::encoder_api::Encoder;
use net_core_api::decoder_api::Decoder;
use net_core_api::typed_api::Typed;

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
        group_id: Option<&str>,
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
    async fn request(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
        jwt: Jwt,
    ) -> Result<Envelope, String> {
        let request_agent_id = enveloped_request.get_agent_id().ok();

        if enveloped_request.get_type() != self.get_requesting_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()));
        }
        let request = HttpOverviewDashboardFiltersRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();

        let executed_query_response = Self::execute_queries(
            connection_pool,
            Some(jwt.get_tenant_id()),
            request_start_date,
            request_end_date,
        ).await;

        if let Err(e) = executed_query_response {
            return Err(format!("error: {:?}", e));
        }
        let response = executed_query_response.unwrap();

        log::info!("Got response on request: {:?}", response);

        let dto_response: HttpOverviewDashboardFiltersDTO = response.into();

        Ok(Envelope::new(
            enveloped_request.get_jwt_token().ok(),
            request_agent_id,
            HttpOverviewDashboardFiltersDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        HttpOverviewDashboardFiltersRequestDTO::get_data_type()
    }
}