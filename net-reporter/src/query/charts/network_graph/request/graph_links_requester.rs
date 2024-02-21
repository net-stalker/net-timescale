use std::sync::Arc;

use net_reporter_api::api::network_graph::network_graph_filters::NetworkGraphFiltersDTO;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::query::charts::network_graph::response::graph_edge::GraphEdgeResponse;


const GRAPH_NODE_REQUEST_QUERY: &str = "
    SELECT src_addr as src_id, dst_addr as dst_id, SUM(packet_length) as value
    FROM network_graph_aggregate
    WHERE 
        group_id = $1
        AND bucket >= $2
        AND bucket < $3
        {}
        {}
    GROUP BY src_addr, dst_addr
    HAVING
        1 = 1
        {}
        {}
    ORDER BY src_addr, dst_addr;
";

const EXCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND not (string_to_array(protocols, ':') && $4)
";

const INCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND (string_to_array(protocols, ':') @> $4)
";

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr IN (SELECT unnest($5)) OR dst_addr IN (SELECT unnest($5)))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr NOT IN (SELECT unnest($5)) AND dst_addr NOT IN (SELECT unnest($5)))
";

const SET_LOWER_BYTES_BOUND: &str = "
    AND SUM(packet_length) >= $6
";

const SET_UPPER_BYTES_BOUND: &str = "
    AND SUM(packet_length) < $7
";

#[derive(Default)]
pub struct GraphLinksRequester {}

impl GraphLinksRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &NetworkGraphFiltersDTO,
    ) -> Result<Vec<GraphEdgeResponse>, Error> {
        let mut request_query = GRAPH_NODE_REQUEST_QUERY.to_string();

        match filters.is_include_protocols_mode() {
            Some(true) => request_query = request_query.replacen("{}", INCLUDE_PROTOCOLS_FILTER_QUERY, 1),
            Some(false) => request_query = request_query.replacen("{}", EXCLUDE_PROTOCOLS_FILTER_QUERY, 1),
            None => request_query = request_query.replacen("{}", "", 1)
        }

        match filters.is_include_endpoints_mode() {
            Some(true) => request_query = request_query.replacen("{}", INCLUDE_ENDPOINT_FILTER_QUERY, 1),
            Some(false) => request_query = request_query.replacen("{}", EXCLUDE_ENDPOINT_FILTER_QUERY, 1),
            None => request_query = request_query.replacen("{}", "", 1)
        };

        match filters.get_bytes_lower_bound() {
            Some(_) => request_query = request_query.replacen("{}", SET_LOWER_BYTES_BOUND, 1),
            None => request_query = request_query.replacen("{}", "", 1)
        };

        match filters.get_bytes_upper_bound() {
            Some(_) => request_query = request_query.replacen("{}", SET_UPPER_BYTES_BOUND, 1),
            None => request_query = request_query.replacen("{}", "", 1)
        };

        let mut sqlx_query = sqlx::query_as(request_query.as_str())
            .bind(group_id)
            .bind(start_date)
            .bind(end_date);

        sqlx_query = match filters.is_include_protocols_mode() {
            Some(_) => sqlx_query.bind(filters.get_protocols()),
            None => sqlx_query,
        };

        sqlx_query = match filters.is_include_endpoints_mode() {
            Some(_) => sqlx_query.bind(filters.get_endpoints()),
            None => sqlx_query,
        };

        sqlx_query = match filters.get_bytes_lower_bound() {
            Some(lower_bound) => sqlx_query.bind(lower_bound),
            None => sqlx_query,
        };

        sqlx_query = match filters.get_bytes_upper_bound() {
            Some(upper_bound) => sqlx_query.bind(upper_bound),
            None => sqlx_query,
        };

        sqlx_query.fetch_all(connection_pool.as_ref())
            .await
    }
}
