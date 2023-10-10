use std::rc::Rc;
use async_std::task::block_on;
use futures::TryStreamExt;
use net_proto_api::api::API;
use net_proto_api::decoder_api::Decoder;
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
        let communication_types = value.concatenated_protocols
            .split(':')
            .map(|protocol| protocol.to_string())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<String>>();
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
pub fn get_network_graph_for_dashboard(transaction: &mut sqlx::Transaction<Postgres>, request: &[u8]) -> Rc<dyn API> {
    let envelope = Envelope::decode(request);
    let address_pairs = block_on(address_pair::transaction_select_address_pairs_by_date_cut(
        transaction,
        &envelope,
    )).unwrap();
    let addresses = block_on(address_info::transaction_select_address_info_by_date_cut(
        transaction,
        &envelope
    )).unwrap();
    let mut edges_dto = Vec::<GraphEdgeDTO>::with_capacity(address_pairs.len());
    let mut nodes_dto = Vec::<GraphNodeDTO>::with_capacity(addresses.len());

    address_pairs.into_iter().for_each(|pair| {
        edges_dto.push(pair.into());
    });
    addresses.into_iter().for_each(|info| {
        nodes_dto.push(info.into());
    });
    Rc::new(NetworkGraphDTO::new(nodes_dto.as_slice(), edges_dto.as_slice()))
}
