use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use net_reporter_api::api::http_responses::http_response::HttpResponseDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpResponseResponse {
    #[sqlx(rename = "frametime")]
    packet_date: DateTime<Utc>,
    #[sqlx(rename = "http_date")]
    http_date: Option<DateTime<Utc>>,
    #[sqlx(rename = "client")]
    client: String,
    #[sqlx(rename = "server")]
    server: String,
    #[sqlx(rename = "response_code")]
    response_code: i64,
}

impl From<HttpResponseResponse> for HttpResponseDTO {
    fn from(value: HttpResponseResponse) -> Self {
        HttpResponseDTO::new(
            value.http_date.map_or(value.packet_date.timestamp_millis(), |date| date.timestamp_millis()),
            value.client.as_str(),
            value.server.as_str(),
            value.response_code,
        )
    }
}
