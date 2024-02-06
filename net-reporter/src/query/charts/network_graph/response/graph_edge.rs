use net_timescale_api::api::network_graph::graph_edge::GraphEdgeDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct GraphEdgeResponse {
    pub src_id: String,
    pub dst_id: String,
    pub concatenated_protocols: String,
}

impl From<GraphEdgeResponse> for GraphEdgeDTO {
    fn from(value: GraphEdgeResponse) -> Self {
        GraphEdgeDTO::new(
            &value.src_id,
            &value.dst_id,
            value.concatenated_protocols
                .split(':')
                .map(|protocol| protocol.to_string())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect::<Vec<String>>()
                .as_slice()
        )
    }
}