use futures::TryStreamExt;
use net_proto_api::envelope::envelope::Envelope;
use sqlx::{
    Pool,
    Postgres
};
use net_timescale_api::api::{
    network_graph::{
        graph_edge::GraphEdgeDTO,
        graph_node::GraphNodeDTO,
        network_graph::NetworkGraphDTO,
    }, network_graph_request::NetworkGraphRequestDTO
};
use crate::repository::address_pair::{AddressPair, self};
use crate::repository::address_info::{AddressInfo, self};

#[derive(Clone)]
pub struct NetworkGraphRequest {
    start_date_time: i64,
    end_date_time: i64,
}

impl NetworkGraphRequest {
    pub fn get_start_date_time(&self) -> i64 {
        self.start_date_time
    }

    pub fn get_end_date_time(&self) -> i64 {
        self.end_date_time
    }
}

impl From<NetworkGraphRequestDTO> for NetworkGraphRequest {
    fn from(val: NetworkGraphRequestDTO) -> Self {
        NetworkGraphRequest {
            start_date_time: val.get_start_date_time(),
            end_date_time: val.get_end_date_time(),
        }
    }
}

impl From<AddressInfo> for GraphNodeDTO {
    fn from(value: AddressInfo) -> GraphNodeDTO {
        GraphNodeDTO::new(&value.node_id, &value.agent_id)
    }
}

impl From<AddressPair> for GraphEdgeDTO {
    fn from(value: AddressPair) -> GraphEdgeDTO {
        let communication_types: Vec<String> = value.communication_types
            .split(':')
            .map(|ty| ty.to_string())
            .collect();
        GraphEdgeDTO::new(&value.src_id, &value.dst_id, &communication_types)
    }
}

pub async fn get_network_graph_by_date_cut(connection: &Pool<Postgres>, envelope: &Envelope) -> NetworkGraphDTO {
    let mut address_pairs = address_pair::select_address_pairs_by_date_cut(
        connection, envelope
    ).await;
    let mut addresses = address_info::select_address_info_by_date_cut(
        connection, envelope
    ).await;

    let mut edges_dto = Vec::default();
    let mut nodes_dto = Vec::default();

    while let Some(pair) = address_pairs.try_next().await.unwrap() {
        edges_dto.push(pair.into());
    }
    while let Some(address) = addresses.try_next().await.unwrap() {
        nodes_dto.push(address.into());
    }

    NetworkGraphDTO::new(nodes_dto.as_slice(), edges_dto.as_slice())
}
