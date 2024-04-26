use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

use net_reporter_api::api::http_responses::http_response::HttpResponseDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpResponseResponse {
    #[sqlx(rename = "Frametime")]
    packet_date: DateTime<Utc>,
    #[sqlx(rename = "Http_Date")]
    http_date: Option<DateTime<Utc>>,
    #[sqlx(rename = "Client")]
    client: String,
    #[sqlx(rename = "Server")]
    server: String,
    #[sqlx(rename = "Response_Code")]
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
