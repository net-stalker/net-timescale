use std::sync::Arc;

use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::TimeZone;
use sqlx::types::chrono::Utc;
use sqlx::Error;
use sqlx::Pool;
use sqlx::Postgres;

use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::typed_api::Typed;

use net_timescale_api::api::network_graph::network_graph::NetworkGraphDTO;
use net_timescale_api::api::network_graph::network_graph_request::NetworkGraphRequestDTO;

use crate::query::charts::network_graph::response::graph_edge::GraphEdgeResponse;
use crate::query::charts::network_graph::response::graph_node::GraphNodeResponse;
use crate::query::charts::network_graph::response::network_graph::NetworkGraphResponse;
use crate::query::requester::Requester;

const GRAPH_NODE_REQUEST_QUERY: &str = "
    SELECT agent_id, node_id
    FROM (
        SELECT DISTINCT agent_id, src_addr AS node_id
        FROM network_graph_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
        UNION
        SELECT DISTINCT agent_id, dst_addr as node_id
        FROM network_graph_aggregate
        WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
    ) AS info
    ORDER BY node_id;
";

const GRAPH_EDGE_REQUEST_QUERY: &str = "
    SELECT src_addr as src_id, dst_addr as dst_id, STRING_AGG(protocols, ':' ORDER BY protocols) AS concatenated_protocols
    FROM network_graph_aggregate
    WHERE group_id = $1 AND bucket >= $2 AND bucket < $3
    GROUP BY src_addr, dst_addr
    ORDER BY src_addr, dst_addr;
";

pub struct NetworkGraphRequester {}

impl NetworkGraphRequester {
    pub fn new() -> Self { Self {  } }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<(Vec<GraphNodeResponse>, Vec<GraphEdgeResponse>), Error> {
        let graph_nodes: Result<Vec<GraphNodeResponse>, Error> = sqlx::query_as(GRAPH_NODE_REQUEST_QUERY)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await;
        if let Err(e) = graph_nodes {
            return Err(e);
        }
        let graph_nodes = graph_nodes.unwrap();

        let graph_edges: Result<Vec<GraphEdgeResponse>, Error> = sqlx::query_as(GRAPH_EDGE_REQUEST_QUERY)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(connection_pool.as_ref())
            .await;
        if let Err(e) = graph_edges {
            return Err(e);
        }
        let graph_edges = graph_edges.unwrap();

        Ok((graph_nodes, graph_edges))
    }
}

#[async_trait::async_trait]
impl Requester for NetworkGraphRequester {
    async fn request(
        &self,
        connection_pool: Arc<Pool<Postgres>>,
        enveloped_request: Envelope
    ) -> Result<Envelope, String> {
        let request_group_id = enveloped_request.get_group_id().ok();
        let request_agent_id = enveloped_request.get_agent_id().ok();

        if enveloped_request.get_type() != self.get_requesting_type() {
            return Err(format!("wrong request is being received: {}", enveloped_request.get_type()));
        }
        let request = NetworkGraphRequestDTO::decode(enveloped_request.get_data());
        let request_start_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_start_date_time()).unwrap();
        let request_end_date: DateTime<Utc> = Utc.timestamp_millis_opt(request.get_end_date_time()).unwrap();

        let executed_query_response = Self::execute_query(
            connection_pool,
            request_group_id,
            request_start_date,
            request_end_date
        ).await;

        if let Err(e) = executed_query_response {
            return Err(format!("error: {:?}", e));
        }
        let executed_query_response = executed_query_response.unwrap();

        let response: NetworkGraphResponse = executed_query_response.into();
        log::info!("Got response on request: {:?}", response);

        let dto_response: NetworkGraphDTO = response.into();

        Ok(Envelope::new(
            request_group_id,
            request_agent_id,
            NetworkGraphDTO::get_data_type(),
            &dto_response.encode()
        ))
    }
    
    fn get_requesting_type(&self) -> &'static str {
        NetworkGraphRequestDTO::get_data_type()
    }
}