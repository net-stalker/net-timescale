use net_reporter_api::api::network_graph::graph_node::GraphNodeDTO;


#[derive(sqlx::FromRow, Clone, Debug)]
pub struct GraphNodeResponse {
    pub node_id: String,
    pub agent_id: String,
}

impl From<GraphNodeResponse> for GraphNodeDTO {
    fn from(value: GraphNodeResponse) -> Self {
        GraphNodeDTO::new(
            &value.node_id,
            &value.agent_id
        )
    }
}