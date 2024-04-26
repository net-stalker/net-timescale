#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpResponseCodeResponse {
    #[sqlx(rename = "Http_Response_Code")]
    pub http_response_code: String,
}