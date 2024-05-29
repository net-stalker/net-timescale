use net_reporter_api::api::network::network::NetworkDTO;

#[derive(sqlx::FromRow, Debug)]
pub struct Network {
    pub id: String,
    pub name: String,
    pub color: String,
}

impl From<Network> for NetworkDTO {
    fn from(value: Network) -> Self {
        NetworkDTO::new(
            &value.id,
            &value.name,
            &value.color,
        )
    }
}
