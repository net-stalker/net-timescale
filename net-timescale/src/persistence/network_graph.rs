use std::rc::Rc;
use async_std::task::block_on;
use chrono::{DateTime, TimeZone, Utc};
use net_proto_api::api::API;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::typed_api::Typed;
use sqlx::{Pool, Postgres, Transaction};
use net_timescale_api::api::{
    network_graph::{
        graph_edge::GraphEdgeDTO,
        graph_node::GraphNodeDTO,
        network_graph::NetworkGraphDTO,
    }, network_graph_request::NetworkGraphRequestDTO
};
use crate::repository::address_pair::AddressPair;
use crate::repository::address_info::AddressInfo;


pub struct NetworkGraph { }

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

// TODO: think about adding a new trait with this methods
impl NetworkGraph {
    pub async fn get_dto(connection: &Pool<Postgres>, data: &Envelope) -> Result<NetworkGraphDTO, String> {
        let group_id = data.get_group_id().ok();
        if data.get_type() != NetworkGraphRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let ng_request = NetworkGraphRequestDTO::decode(data.get_data());
        let start_date: DateTime<Utc> = Utc.timestamp_nanos(ng_request.get_start_date_time());
        let end_date: DateTime<Utc> = Utc.timestamp_nanos(ng_request.get_end_date_time());

        let address_pairs = match AddressPair::select_by_date_cut(
            connection, group_id, start_date, end_date
        ).await {
            Ok(pairs) => pairs,
            Err(err) => return Err(format!("couldn't query address pairs: {}", err)),
        };
        let addresses = match AddressInfo::select_by_date_cut(
            connection, group_id, start_date, end_date
        ).await {
            Ok(addresses) => addresses,
            Err(err) => return Err(format!("couldn't query info about addresses: {}", err)),
        };

        let mut edges_dto = Vec::<GraphEdgeDTO>::with_capacity(address_pairs.len());
        let mut nodes_dto = Vec::<GraphNodeDTO>::with_capacity(addresses.len());

        address_pairs.into_iter().for_each(|pair| {
            edges_dto.push(pair.into());
        });
        addresses.into_iter().for_each(|info| {
            nodes_dto.push(info.into());
        });

        Ok(NetworkGraphDTO::new(nodes_dto.as_slice(), edges_dto.as_slice()))
    }
    pub async fn transaction_get_dto(
        transaction: &mut Transaction<'_, Postgres>,
        data: &Envelope
    ) -> Result<NetworkGraphDTO, String>
    {
        let group_id = data.get_group_id().ok();
        if data.get_type() != NetworkGraphRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let ng_request = NetworkGraphRequestDTO::decode(data.get_data());
        let start_date: DateTime<Utc> = Utc.timestamp_nanos(ng_request.get_start_date_time());
        let end_date: DateTime<Utc> = Utc.timestamp_nanos(ng_request.get_end_date_time());

        let address_pairs = match AddressPair::transaction_select_by_date_cut(
            transaction, group_id, start_date, end_date
        ).await {
            Ok(pairs) => pairs,
            Err(err) => return Err(format!("couldn't query address pairs: {}", err)),
        };
        let addresses = match AddressInfo::transaction_select_by_date_cut(
            transaction, group_id, start_date, end_date
        ).await {
            Ok(addresses) => addresses,
            Err(err) => return Err(format!("couldn't query info about addresses: {}", err)),
        };

        let mut edges_dto = Vec::<GraphEdgeDTO>::with_capacity(address_pairs.len());
        let mut nodes_dto = Vec::<GraphNodeDTO>::with_capacity(addresses.len());

        address_pairs.into_iter().for_each(|pair| {
            edges_dto.push(pair.into());
        });
        addresses.into_iter().for_each(|info| {
            nodes_dto.push(info.into());
        });

        Ok(NetworkGraphDTO::new(nodes_dto.as_slice(), edges_dto.as_slice()))
    }
}

// TODO: having trait with method transaction_get_dto we can easily derive this method
impl super::ChartGenerator for NetworkGraph {
    fn generate_chart(transaction: &mut Transaction<Postgres>, data: &Envelope) -> Result<Rc<dyn API>, String> {
        match block_on(Self::transaction_get_dto(transaction, data)) {
            Ok(ng_dto) => Ok(Rc::new(ng_dto)),
            Err(err) => Err(err)
        }
    }
}
