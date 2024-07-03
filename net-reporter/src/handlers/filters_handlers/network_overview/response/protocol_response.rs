#[derive(sqlx::FromRow, Clone, Debug)]
pub struct ProtocolResponse {
    #[sqlx(rename = "protocol")]
    pub protocol: String,
}
