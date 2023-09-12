use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use sqlx::{
    Pool,
    Postgres
};
use net_timescale_api::api::{
    network_packet::NetworkPacketDTO,
    network_graph::{
        graph_edge::GraphEdgeDTO,
        graph_node::GraphNodeDTO,
        network_graph::NetworkGraphDTO,
    }
};
use crate::repository::address_pair::{AddressPair, self};
use crate::repository::address_info::{AddressInfo, self};


impl From<AddressInfo> for GraphNodeDTO {
    fn from(value: AddressInfo) -> GraphNodeDTO {
        GraphNodeDTO::new(&value.id, &value.aggregator)
    }
}

impl From<AddressPair> for GraphEdgeDTO {
    fn from(value: AddressPair) -> GraphEdgeDTO {
        GraphEdgeDTO::new(&value.src_id, &value.dst_id)
    }
}

pub async fn get_network_graph_by_date_cut(connection: &Pool<Postgres>, date_start: DateTime<Utc>,
                                     date_end: DateTime<Utc>) -> NetworkGraphDTO {
    let mut address_pairs = address_pair::select_address_pairs_by_date_cut(
        connection, date_start, date_end
    ).await;
    let mut addresses = address_info::select_address_info_by_date_cut(
        connection, date_start, date_end
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
