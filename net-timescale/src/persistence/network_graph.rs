use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use sqlx::{
    Pool,
    Postgres
};
use net_timescale_api::api::{
    network_graph::{
        graph_edge::GraphEdgeDTO,
        graph_node::GraphNodeDTO,
        network_graph::NetworkGraphDTO,
    }
};
use crate::repository::address_pair::{AddressPair, self};
use crate::repository::address_info::{AddressInfo, self};
use crate::repository::{captured_traffic, realtime_client};


impl Into<GraphNodeDTO> for AddressInfo {
    fn into(self) -> GraphNodeDTO {
        GraphNodeDTO::new(&self.addr)
    }
}

impl Into<GraphEdgeDTO> for AddressPair {
    fn into(self) -> GraphEdgeDTO {
        GraphEdgeDTO::new(&self.src_addr, &self.dst_addr)
    }
}

pub async fn get_network_graph_by_date_cut(
    pool: &Pool<Postgres>, date_start: DateTime<Utc>,
    date_end: DateTime<Utc>, client_id: i64
) -> NetworkGraphDTO
{
    let mut transaction = pool.begin().await.unwrap();
    if date_end.timestamp_nanos() == 0 {
        // real-time
        let last_index_in_captured_traffic =
            captured_traffic::get_max_id(&mut transaction).await.unwrap();
        match realtime_client::check_client_id_existence(&mut transaction, client_id).await {
            Ok(_) => {
                realtime_client::delete_client(&mut transaction, client_id).await.unwrap();
            },
            Err(_) => {
                realtime_client::insert_client(&mut transaction, client_id, last_index_in_captured_traffic).await.unwrap();
            }
        }
    }
    let address_pairs = address_pair::select_address_pairs_by_date_cut_transaction(
        &mut transaction, date_start, date_end
    ).await.unwrap();
    let addresses = address_info::select_address_info_by_date_cut_transaction(
        &mut transaction, date_start, date_end
    ).await.unwrap();

    transaction.commit().await.unwrap();

    let mut edges_dto = Vec::with_capacity(address_pairs.len());
    let mut nodes_dto = Vec::with_capacity(addresses.len());

    for pair in address_pairs.into_iter() {
        edges_dto.push(pair.into());
    }

    for node in addresses.into_iter() {
        nodes_dto.push(node.into());
    }

    NetworkGraphDTO::new(nodes_dto.as_slice(), edges_dto.as_slice())
}