use net_reporter_api::api::http_request_methods_dist::http_request::HttpRequestDTO;


#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpRequestResponse {
    name: String,
    amount: i64
}

impl From<HttpRequestResponse> for HttpRequestDTO {
    fn from(value: HttpRequestResponse) -> Self {
        HttpRequestDTO::new(
            value.name.as_str(),
            value.amount,
        )
    }
}