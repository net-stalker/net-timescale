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

    async fn get_query_based_on_requested_filters(filters: &NetworkGraphFiltersDTO) -> String {
        let mut placeholder_value = 4;
        let mut request_query = GRAPH_NODE_REQUEST_QUERY.to_owned();

        match filters.is_include_protocols_mode() {
            Some(true) => {
                let protocols_query = INCLUDE_PROTOCOLS_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", protocols_query.as_str(), 1);
            },
            Some(false) => {
                let protocols_query = EXCLUDE_PROTOCOLS_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", protocols_query.as_str(), 1);
            },
            None => request_query = request_query.replacen("{}", "", 1)
        }

        match filters.is_include_endpoints_mode() {
            Some(true) => {
                let endpoints_query = INCLUDE_ENDPOINT_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", endpoints_query.as_str(), 1);
            },
            Some(false) => { 
                let endpoints_query = EXCLUDE_ENDPOINT_FILTER_QUERY.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", endpoints_query.as_str(), 1) 
            },
            None => request_query = request_query.replacen("{}", "", 1)
        };

        match filters.get_bytes_lower_bound() {
            Some(_) => {
                let lower_bytes_query = SET_LOWER_BYTES_BOUND.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                placeholder_value += 1;
                request_query = request_query.replacen("{}", lower_bytes_query.as_str(), 1)
            },
            None => request_query = request_query.replacen("{}", "", 1)
        };

        match filters.get_bytes_upper_bound() {
            Some(_) => {
                let upper_bytes_query = SET_UPPER_BYTES_BOUND.to_string().replace("{}", format!("${}", placeholder_value).as_str());
                request_query = request_query.replacen("{}", upper_bytes_query.as_str(), 1)
            },
            None => request_query = request_query.replacen("{}", "", 1)
        };
        request_query
    }

    pub async fn execute_query(
        connection_pool: Arc<Pool<Postgres>>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        filters: &NetworkGraphFiltersDTO,
    ) -> Result<Vec<GraphEdgeResponse>, Error> {
        let request_query = Self::get_query_based_on_requested_filters(filters).await;

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
