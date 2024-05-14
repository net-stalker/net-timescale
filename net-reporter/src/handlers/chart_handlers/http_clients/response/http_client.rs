use net_reporter_api::api::http_clients::http_client::HttpClientDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpClientResponse {
    #[sqlx(rename = "Endpoint")]
    endpoint: String,
    #[sqlx(rename = "User_Agent")]
    user_agent: Option<String>,
    #[sqlx(rename = "Requests")]
    requests: i64,
}

impl From<HttpClientResponse> for HttpClientDTO {
    fn from(value: HttpClientResponse) -> Self {
        HttpClientDTO::new(
            &value.endpoint,
            value.user_agent.as_deref(),
            value.requests,
        )
    }
}