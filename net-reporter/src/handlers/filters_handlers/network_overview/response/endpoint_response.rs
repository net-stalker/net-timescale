#[derive(sqlx::FromRow, Clone, Debug)]
pub struct EndpointResponse {
    #[sqlx(rename = "endpoint")]
    pub endpoint: String,
}
