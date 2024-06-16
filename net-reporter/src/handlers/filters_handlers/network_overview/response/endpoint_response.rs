#[derive(sqlx::FromRow, Clone, Debug)]
pub struct EndpointResponse {
    #[sqlx(rename = "Endpoint")]
    pub endpoint: String,
}
