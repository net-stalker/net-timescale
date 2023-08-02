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
use crate::repository::realtime_client;


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

pub async fn get_network_graph_by_date_cut(pool: &Pool<Postgres>, date_start: DateTime<Utc>,
                                           date_end: DateTime<Utc>) -> NetworkGraphDTO {
    let mut address_pairs = address_pair::select_address_pairs_by_date_cut(
        pool, date_start, date_end
    ).await;
    let mut addresses = address_info::select_address_info_by_date_cut(
        pool, date_start, date_end
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

pub async fn get_network_graph_and_handle_client_realtime(
    pool: &Pool<Postgres>, date_start: DateTime<Utc>,
    date_end: DateTime<Utc>, client_id: i64
) -> (NetworkGraphDTO, i64)
{
    let mut transaction = pool.begin().await.unwrap();
    let mut address_pairs = address_pair::select_address_pairs_by_date_cut_transaction(
        &mut transaction, date_start, date_end
    ).await;
    let mut addresses = address_info::select_address_info_by_date_cut_transaction(
        &mut transaction, date_start, date_end
    ).await;
    let mock_index = 90;
    realtime_client::update_last_index(&mut transaction, client_id, mock_index).await.unwrap();

    transaction.commit().await.unwrap();

    todo!()
}