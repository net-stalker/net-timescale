use net_reporter_api::api::network_graph::graph_edge::GraphEdgeDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct GraphEdgeResponse {
    #[sqlx(rename = "src_ip")]
    pub src_id: String,
    #[sqlx(rename = "dst_ip")]
    pub dst_id: String,
    #[sqlx(rename = "value")]
    pub value: i64,
}

impl From<GraphEdgeResponse> for GraphEdgeDTO {
    fn from(value: GraphEdgeResponse) -> Self {
        GraphEdgeDTO::new(
            &value.src_id,
            &value.dst_id,
            value.value,
        )
    }
}