#[derive(sqlx::FromRow, Clone, Debug)]
pub struct EndpointResponse {
    pub endpoint: String,
}
