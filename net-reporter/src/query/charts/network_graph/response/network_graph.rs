use net_timescale_api::api::network_graph::graph_edge::GraphEdgeDTO;
use net_timescale_api::api::network_graph::graph_node::GraphNodeDTO;
use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;

use super::graph_node::GraphNodeResponse;
use super::graph_edge::GraphEdgeResponse;

#[derive(Default, Clone, Debug)]
pub struct NetworkGraphResponse {
    graph_nodes: Vec<GraphNodeResponse>,
    graph_edges: Vec<GraphEdgeResponse>,
}

impl From<NetworkGraphResponse> for NetworkGraphDTO {
    fn from(value: NetworkGraphResponse) -> Self {
        NetworkGraphDTO::new(
            value.graph_nodes
                .into_iter()
                .map(| bandwidth_bucket | bandwidth_bucket.into())
                .collect::<Vec<GraphNodeDTO>>()
                .as_slice(),
            value.graph_edges
            .into_iter()
            .map(| bandwidth_bucket | bandwidth_bucket.into())
            .collect::<Vec<GraphEdgeDTO>>()
            .as_slice()
        )
    }
}

impl From<(Vec<GraphNodeResponse>, Vec<GraphEdgeResponse>)> for NetworkGraphResponse {
    fn from(value: (Vec<GraphNodeResponse>, Vec<GraphEdgeResponse>)) -> Self {
        NetworkGraphResponse {
            graph_nodes: value.0,
            graph_edges: value.1
        }
    }
}