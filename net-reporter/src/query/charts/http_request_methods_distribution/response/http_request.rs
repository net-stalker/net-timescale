use net_reporter_api::api::http_request_methods_distribution::http_request_method::HttpRequestMethodDTO;


#[derive(sqlx::FromRow, Clone, Debug)]
pub struct HttpRequestMethodResponse {
    name: String,
    amount: i64
}

impl From<HttpRequestMethodResponse> for HttpRequestMethodDTO {
    fn from(value: HttpRequestMethodResponse) -> Self {
        HttpRequestMethodDTO::new(
            value.name.as_str(),
            value.amount,
        )
    }
}