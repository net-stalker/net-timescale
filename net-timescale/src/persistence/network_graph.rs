use std::rc::Rc;
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
use crate::persistence::{ChartGenerator, Persistence};
use crate::repository::address_pair::AddressPair;
use crate::repository::address_info::AddressInfo;

#[derive(Default, Debug)]
pub struct PersistenceNetworkGraph { }

impl PersistenceNetworkGraph {
    // TODO: rename `into_inner` to `into_wrapped`
    pub fn into_wrapped(self) -> Rc<dyn ChartGenerator> {
        Rc::new(self)
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

#[async_trait::async_trait]
impl Persistence for PersistenceNetworkGraph {
    // TODO: use template instead of using Pool and transaction is separate methods

    async fn get_chart_dto(
        &self,
        connection: &Pool<Postgres>,
        data: &Envelope,
    ) -> Result<Rc<dyn API>, String> {
        let group_id = data.get_group_id().ok();
        if data.get_type() != NetworkGraphRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let ng_request = NetworkGraphRequestDTO::decode(data.get_data());
        // TODO: take a look at #8692yt2vj
        let start_date: DateTime<Utc> = Utc.timestamp_millis_opt(ng_request.get_start_date_time()).unwrap();
        let end_date: DateTime<Utc> = Utc.timestamp_millis_opt(ng_request.get_end_date_time()).unwrap();

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

        Ok(Rc::new(NetworkGraphDTO::new(nodes_dto.as_slice(), edges_dto.as_slice())))
    }
    async fn transaction_get_chart_dto(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        data: &Envelope
    ) -> Result<Rc<dyn API>, String>
    {
        let group_id = data.get_group_id().ok();
        if data.get_type() != NetworkGraphRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let ng_request = NetworkGraphRequestDTO::decode(data.get_data());
        // TODO: take a look at #8692yt2vj
        let start_date: DateTime<Utc> = Utc.timestamp_millis_opt(ng_request.get_start_date_time()).unwrap();
        let end_date: DateTime<Utc> = Utc.timestamp_millis_opt(ng_request.get_end_date_time()).unwrap();

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

        Ok(Rc::new(NetworkGraphDTO::new(nodes_dto.as_slice(), edges_dto.as_slice())))
    }
}

// TODO: having trait with method transaction_get_dto we can easily derive this method
impl ChartGenerator for PersistenceNetworkGraph {
    fn get_requesting_type(&self) -> &'static str where Self: Sized {
        // TODO: this method can also be derived somehow, probably by adding parameters into derive macro
        NetworkGraphRequestDTO::get_data_type()
    }
}
