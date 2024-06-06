use net_reporter_api::api::network::network_id::NetworkIdDTO;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct NetworkId {
    pub id: String,
}

impl From<NetworkId> for NetworkIdDTO {
    fn from(value: NetworkId) -> Self {
        NetworkIdDTO::new(&value.id)
    }
}
