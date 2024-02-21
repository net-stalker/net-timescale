use net_reporter_api::api::network_graph::graph_node::GraphNodeDTO;


#[derive(sqlx::FromRow, Clone, Debug)]
pub struct GraphNodeResponse {
    pub node_id: String,
    pub value: i64,
    // we don't need agent_id here, so just remove it
}

impl From<GraphNodeResponse> for GraphNodeDTO {
    fn from(value: GraphNodeResponse) -> Self {
        todo!("udpate `GraphNodeDTO`");
        GraphNodeDTO::new(
            &value.node_id,
            value.value,
        )
    }
}