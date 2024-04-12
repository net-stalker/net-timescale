use std::sync::Arc;

use net_reporter_api::api::network_graph::network_graph_filters::NetworkGraphFiltersDTO;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::{query::charts::network_graph::response::graph_edge::GraphEdgeResponse, query_builder::{query_builder::QueryBuilder, sqlx_query_builder_wrapper::SqlxQueryBuilderWrapper}};


const GRAPH_LINKS_REQUEST_QUERY: &str = "
    SELECT src_addr as src_id, dst_addr as dst_id, SUM(packet_length) as value
    FROM network_graph_aggregate
    WHERE 
        tenant_id = $1
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
    AND not (string_to_array(protocols, ':') && {})
";

const INCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND (string_to_array(protocols, ':') @> {})
";

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr IN (SELECT unnest({})) OR dst_addr IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (src_addr NOT IN (SELECT unnest({})) AND dst_addr NOT IN (SELECT unnest({})))
";

const SET_LOWER_BYTES_BOUND: &str = "
    AND SUM(packet_length) >= {}
";

const SET_UPPER_BYTES_BOUND: &str = "
    AND SUM(packet_length) < {}
";

#[derive(Default)]
pub struct GraphLinksRequester {}

impl GraphLinksRequester {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        tenant_id: &str,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &NetworkGraphFiltersDTO,
    ) -> Result<Vec<GraphEdgeResponse>, Error> {
        let query_string = QueryBuilder::new(GRAPH_LINKS_REQUEST_QUERY, 4)
            .add_dynamic_filter(filters.is_include_protocols_mode(), 1, INCLUDE_PROTOCOLS_FILTER_QUERY, EXCLUDE_PROTOCOLS_FILTER_QUERY)
            .add_dynamic_filter(filters.is_include_endpoints_mode(), 1, INCLUDE_ENDPOINT_FILTER_QUERY, EXCLUDE_ENDPOINT_FILTER_QUERY)
            .add_static_filter(filters.get_bytes_lower_bound(), SET_LOWER_BYTES_BOUND, 1)
            .add_static_filter(filters.get_bytes_upper_bound(), SET_UPPER_BYTES_BOUND, 1)
            .build_query();

        SqlxQueryBuilderWrapper::<GraphEdgeResponse>::new(query_string.as_str())
            .add_param(tenant_id)
            .add_param(start_date)
            .add_param(end_date)
            .add_option_param(filters.is_include_protocols_mode().map(|_| filters.get_protocols().to_vec()))
            .add_option_param(filters.is_include_endpoints_mode().map(|_| filters.get_endpoints().to_vec()))
            .add_option_param(filters.get_bytes_lower_bound())
            .add_option_param(filters.get_bytes_upper_bound())
            .execute_query(connection_pool).await
    }
}
