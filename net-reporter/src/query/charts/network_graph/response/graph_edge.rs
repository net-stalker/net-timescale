use net_reporter_api::api::network_graph::graph_edge::GraphEdgeDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct GraphEdgeResponse {
    pub src_id: String,
    pub dst_id: String,
    pub value: i64,
}

impl From<GraphEdgeResponse> for GraphEdgeDTO {
    fn from(value: GraphEdgeResponse) -> Self {
        todo!("update GraphEdgeDTO");
        GraphEdgeDTO::new(
            &value.src_id,
            &value.dst_id,
            value.value,
        )
    }
}