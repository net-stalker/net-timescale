use chrono::{DateTime, Utc};
use diesel::PgConnection;
use net_timescale_api::api::network_graph::graph_edge::GraphEdgeDTO;
use net_timescale_api::api::network_graph::graph_node::GraphNodeDTO;
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use crate::repository::address_pair::{AddressPair, self};
use crate::repository::address_info::{AddressInfo, self};


impl Into<GraphNodeDTO> for AddressInfo {
    fn into(self) -> GraphNodeDTO {
        GraphNodeDTO::new(self.addr)
    }
}

impl Into<GraphEdgeDTO> for AddressPair {
    fn into(self) -> GraphEdgeDTO {
        GraphEdgeDTO::new(self.src_addr, self.dst_addr)
    }
}

pub fn get_network_graph_by_date_cut(connection: &mut PgConnection, date_start: DateTime<Utc>,
                                     date_end: DateTime<Utc>) -> NetworkGraphDTO {
    let address_pairs = address_pair::select_address_pairs_by_date_cut(
        connection, date_start, date_end
    );
    let addresses = address_info::select_address_info_by_date_cut(
        connection, date_start, date_end
    );
    let mut edges_dto = Vec::with_capacity(address_pairs.len());
    let mut nodes_dto = Vec::with_capacity(addresses.len());
    for pair in address_pairs.into_iter() {
        edges_dto.push(pair.into());
    }
    for address in addresses.into_iter() {
        nodes_dto.push(address.into());
    }
    NetworkGraphDTO::new(nodes_dto, edges_dto)
}