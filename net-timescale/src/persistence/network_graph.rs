use chrono::{DateTime, TimeZone, Utc};
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
use net_timescale_api::api::network_graph_request::NetworkGraphRequestDTO;
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

#[derive(Debug, PartialEq, Clone)]
pub struct NetworkGraphRequest {
    pub start_date_time: DateTime<Utc>,
    pub end_date_time: DateTime<Utc>,
    pub is_subscribe: bool
}

impl Into<NetworkGraphRequest> for NetworkGraphRequestDTO {
    fn into(self) -> NetworkGraphRequest {
        let start_date_time = Utc.timestamp_millis_opt(self.get_start_date_time()).unwrap();
        let end_date_time = Utc.timestamp_millis_opt(self.get_end_date_time()).unwrap();

        NetworkGraphRequest {
            start_date_time,
            end_date_time,
            is_subscribe: self.is_subscribe(),
        }
    }
}

// TODO: write tests
pub async fn handle_realtime_request(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    ng_request: &NetworkGraphRequest,
    connection_id: i64,
)
{
    if ng_request.end_date_time.timestamp_nanos() != 0 {
        return;
    }
    // real-time
    match realtime_client::check_client_id_existence(transaction, connection_id).await {
        Ok(_) => {
            match ng_request.is_subscribe {
                true => {
                    let last_index_in_captured_traffic =
                        captured_traffic::get_max_id(transaction).await.unwrap() as i64;
                    realtime_client::update_last_index(
                        transaction,
                        connection_id,
                        last_index_in_captured_traffic)
                        .await
                        .unwrap();
                },
                false => {
                    realtime_client::delete_client(transaction, connection_id).await.unwrap();
                }
            }
        },
        Err(_) => {
            match ng_request.is_subscribe {
                true => {
                    let last_index_in_captured_traffic =
                        captured_traffic::get_max_id(transaction).await.unwrap() as i64;

                    realtime_client::insert_client(transaction, connection_id, last_index_in_captured_traffic).await.unwrap();
                },
                false => {
                    log::error!("there is no connection {} in db, can't delete it", connection_id);
                }
            }
        }
    }
}

pub async fn reply_network_graph_request(
    pool: &Pool<Postgres>,
    ng_request: NetworkGraphRequest,
    connection_id: i64
) -> NetworkGraphDTO
{
    let mut transaction = pool.begin().await.unwrap();

    handle_realtime_request(&mut transaction, &ng_request, connection_id).await;

    let start_date = ng_request.start_date_time;
    let end_date = ng_request.end_date_time;

    let address_pairs = address_pair::select_address_pairs_by_date_cut_transaction(
        &mut transaction, start_date, end_date
    ).await.unwrap();
    let addresses = address_info::select_address_info_by_date_cut_transaction(
        &mut transaction, start_date, end_date
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

pub async fn get_network_graph_by_index(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    index: i64
) -> Result<NetworkGraphDTO, sqlx::Error>
{
    let address_pairs = address_pair::select_address_pairs_by_index_transaction(
        transaction,
        index
    ).await?;
    let addresses = address_info::select_address_info_by_index_transaction(
        transaction,
        index
    ).await?;
    let mut edges_dto = Vec::with_capacity(address_pairs.len());
    let mut nodes_dto = Vec::with_capacity(addresses.len());

    for pair in address_pairs.into_iter() {
        edges_dto.push(pair.into());
    }

    for node in addresses.into_iter() {
        nodes_dto.push(node.into());
    }

    Ok(NetworkGraphDTO::new(nodes_dto.as_slice(), edges_dto.as_slice()))
}