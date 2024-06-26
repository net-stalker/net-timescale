#[derive(sqlx::FromRow, Clone, Debug)]
pub struct ProtocolResponse {
    #[sqlx(rename = "Protocol")]
    pub protocol: String,
}
