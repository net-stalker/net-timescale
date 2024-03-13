#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpRequestMethodResponse {
    pub http_request_method: String,
}
