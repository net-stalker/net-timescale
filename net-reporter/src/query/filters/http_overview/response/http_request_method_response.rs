#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpRequestMethodResponse {
    #[sqlx(rename = "Http_Request_Method")]
    pub http_request_method: String,
}
