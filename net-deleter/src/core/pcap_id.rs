#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PcapId {
    pub id: String,
}
