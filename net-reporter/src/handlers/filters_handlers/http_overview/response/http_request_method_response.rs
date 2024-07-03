#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpRequestMethodResponse {
    #[sqlx(rename = "http_eequest_method")]
    pub http_request_method: String,
}
