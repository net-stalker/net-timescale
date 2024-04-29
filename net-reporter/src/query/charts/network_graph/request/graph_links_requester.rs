use std::sync::Arc;

use net_reporter_api::api::network_graph::network_graph_filters::NetworkGraphFiltersDTO;
use sqlx::{types::chrono::{DateTime, Utc}, Error, Pool, Postgres};

use crate::{query::charts::network_graph::response::graph_edge::GraphEdgeResponse, query_builder::{query_builder::QueryBuilder, sqlx_query_builder_wrapper::SqlxQueryBuilderWrapper}};


const GRAPH_LINKS_REQUEST_QUERY: &str = "
    SELECT Src_IP, Dst_IP, SUM(Packet_Length) AS Value
    FROM Network_Graph_Materialized_View
    WHERE 
        Tenant_ID = $1
        AND Frametime >= $2
        AND Frametime < $3
        AND Network_ID = $4
        {}
        {}
    GROUP BY Src_IP, Dst_IP
    HAVING
        1 = 1
        {}
        {}
    ORDER BY Src_IP, Dst_IP;
";

const EXCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND NOT (string_to_array(Protocols, ':') && {})
";

const INCLUDE_PROTOCOLS_FILTER_QUERY: &str = "
    AND (string_to_array(Protocols, ':') @> {})
";

const INCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (Src_IP IN (SELECT unnest({})) OR Dst_IP IN (SELECT unnest({})))
";

const EXCLUDE_ENDPOINT_FILTER_QUERY: &str = "
    AND (Src_IP NOT IN (SELECT unnest({})) AND Dst_IP NOT IN (SELECT unnest({})))
";

const SET_LOWER_BYTES_BOUND: &str = "
    AND SUM(Packet_Length) >= {}
";

const SET_UPPER_BYTES_BOUND: &str = "
    AND SUM(Packet_Length) < {}
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
        network_id: i64,
        filters: &NetworkGraphFiltersDTO,
    ) -> Result<Vec<GraphEdgeResponse>, Error> {
        let query_string = QueryBuilder::new(GRAPH_LINKS_REQUEST_QUERY, 5)
            .add_dynamic_filter(filters.is_include_protocols_mode(), 1, INCLUDE_PROTOCOLS_FILTER_QUERY, EXCLUDE_PROTOCOLS_FILTER_QUERY)
            .add_dynamic_filter(filters.is_include_endpoints_mode(), 1, INCLUDE_ENDPOINT_FILTER_QUERY, EXCLUDE_ENDPOINT_FILTER_QUERY)
            .add_static_filter(filters.get_bytes_lower_bound(), SET_LOWER_BYTES_BOUND, 1)
            .add_static_filter(filters.get_bytes_upper_bound(), SET_UPPER_BYTES_BOUND, 1)
            .build_query();

        SqlxQueryBuilderWrapper::<GraphEdgeResponse>::new(query_string.as_str())
            .add_param(tenant_id)
            .add_param(start_date)
            .add_param(end_date)
            .add_param(network_id)
            .add_option_param(filters.is_include_protocols_mode().map(|_| filters.get_protocols().to_vec()))
            .add_option_param(filters.is_include_endpoints_mode().map(|_| filters.get_endpoints().to_vec()))
            .add_option_param(filters.get_bytes_lower_bound())
            .add_option_param(filters.get_bytes_upper_bound())
            .execute_query(connection_pool).await
    }
}
