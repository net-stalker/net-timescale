use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use net_reporter_api::api::http_responses::http_response::HttpResponseDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpResponseResponse {
    date: Option<DateTime<Utc>>,
    client: String,
    server: String,
    response_code: i64,
}

impl From<HttpResponseResponse> for HttpResponseDTO {
    fn from(value: HttpResponseResponse) -> Self {
        HttpResponseDTO::new(
            value.date.map(|date| date.timestamp_millis()),
            value.client.as_str(),
            value.server.as_str(),
            value.response_code,
        )
    }
}
