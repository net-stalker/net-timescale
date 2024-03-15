#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpResponseCodeResponse {
    pub http_response_code: String,
}
