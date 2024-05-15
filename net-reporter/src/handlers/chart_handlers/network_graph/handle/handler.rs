use std::sync::Arc;

use net_component::handler::network_service_handler::NetworkServiceHandler;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use net_reporter_api::api::network_graph::network_graph::NetworkGraphDTO;
use net_reporter_api::api::network_graph::network_graph_filters::NetworkGraphFiltersDTO;
use net_reporter_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;

use crate::handlers::chart_handlers::network_graph::response::graph_edge::GraphEdgeResponse;
use crate::handlers::chart_handlers::network_graph::response::graph_node::GraphNodeResponse;
use crate::handlers::chart_handlers::network_graph::response::network_graph::NetworkGraphResponse;


use super::graph_links_handler::GraphLinksHandler;


#[derive(Default)]
pub struct NetworkGraphHandler {}

impl NetworkGraphHandler {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn get_nodes_from_edges(graph_edges: &[GraphEdgeResponse]) -> Vec<GraphNodeResponse> {
        let mut nodes: Vec<GraphNodeResponse> = Vec::new();
        let mut nodes_map: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for edge in graph_edges {
            nodes_map.insert(&edge.src_id);
            nodes_map.insert(&edge.dst_id);
        }

        nodes_map.into_iter().for_each(|node_id| nodes.push(GraphNodeResponse { node_id: node_id.to_string() }));

        nodes
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &NetworkGraphFiltersDTO,
    ) -> Result<(Vec<GraphNodeResponse>, Vec<GraphEdgeResponse>), Error> {
        let graph_links = GraphLinksHandler::execute_query(
            connection_pool.clone(),
            tenant_id,
            start_date,
            end_date,
            filters
        ).await?;
        let graph_nodes = Self::get_nodes_from_edges(&graph_links).await;

        Ok((graph_nodes, graph_links))
    }
}

#[async_trait::async_trait]
impl NetworkServiceHandler for NetworkGraphHandler {
    async fn handle(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope,
    ) -> Result<Envelope, Box<dyn std::error::Error + Send + Sync>> {
        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != self.get_handler_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()).into());
        }
        let request = NetworkGraphRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();
        let filters: &NetworkGraphFiltersDTO = request.get_filters();

        let executed_query_response = Self::execute_query(
            connection_pool,
            tenant_id,
            request_start_date,
            request_end_date,
            filters,
        ).await?;

        let response: NetworkGraphResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkGraphDTO = response.into();

        Ok(Envelope::new(
            tenant_id,
            NetworkGraphDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_handler_type(&self) -> String {
        NetworkGraphRequestDTO::get_data_type().to_string()
    }
}