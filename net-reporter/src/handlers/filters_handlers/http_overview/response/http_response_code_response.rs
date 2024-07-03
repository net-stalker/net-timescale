#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpResponseCodeResponse {
    #[sqlx(rename = "http_response_code")]
    pub http_response_code: String,
}