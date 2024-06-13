use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Error;
use sqlx::postgres::PgQueryResult;

use crate::materialized_view::manager::manager::MaterializedViewManager;
use crate::materialized_view::query::http_clients::HttpClientsMaterialiazedView;
use crate::materialized_view::query::http_overview_filters::HttpOverviewFiltersMaterializedView;
use crate::materialized_view::query::http_request_methods_distribution::HttpRequestMethodsDistributionMaterializedView;
use crate::materialized_view::query::http_responses::HttpResponsesMaterializedView;
use crate::materialized_view::query::network_bandwidth::NetworkBandwidthMaterializedView;
use crate::materialized_view::query::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointMaterializedView;
use crate::materialized_view::query::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolMaterializedView;
use crate::materialized_view::query::network_graph::NetworkGraphMaterializedView;
use crate::materialized_view::query::network_overview_filters::NetworkOverviewFiltersMaterializedView;
use crate::materialized_view::query::network_packets::NetworkPacketsMaterializedView;
use crate::materialized_view::query::total_http_requests::TotalHttpRequestsMaterializedView;

pub trait MaterializedViewQueries: Send + Sync {
    fn get_name(&self) -> String;

    fn get_creation_query(&self) -> String;

    fn get_refresh_query_blocking(&self) -> String {
        format!("REFRESH MATERIALIZED VIEW {};", self.get_name())
    }

    fn get_refresh_query_concurrent(&self) -> String {
        format!("REFRESH MATERIALIZED VIEW CONCURRENTLY {};", self.get_name())
    }
}

#[async_trait::async_trait]
pub trait MaterializedView: MaterializedViewQueries {

    async fn create(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let create_query = self.get_creation_query();
        sqlx::query(&create_query)
            .execute(pool)
            .await
    }

    async fn refresh_blocking(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let refresh_query = self.get_refresh_query_blocking();
        sqlx::query(&refresh_query)
            .execute(pool)
            .await
    }

    async fn refresh_concurrently(
        &self,
        pool: &Pool<Postgres>
    ) -> Result<PgQueryResult, Error> {
        let refresh_query = self.get_refresh_query_concurrent();
        sqlx::query(&refresh_query)
            .execute(pool)
            .await
    }
}

pub fn get_common_materialized_view_manager() -> MaterializedViewManager {
    MaterializedViewManager::builder()
        .add_materialized_view(Box::<HttpClientsMaterialiazedView>::default())
        .add_materialized_view(Box::<HttpOverviewFiltersMaterializedView>::default())
        .add_materialized_view(Box::<HttpRequestMethodsDistributionMaterializedView>::default())
        .add_materialized_view(Box::<HttpResponsesMaterializedView>::default())
        .add_materialized_view(Box::<NetworkBandwidthPerEndpointMaterializedView>::default())
        .add_materialized_view(Box::<NetworkBandwidthPerProtocolMaterializedView>::default())
        .add_materialized_view(Box::<NetworkBandwidthMaterializedView>::default())
        .add_materialized_view(Box::<NetworkGraphMaterializedView>::default())
        .add_materialized_view(Box::<NetworkOverviewFiltersMaterializedView>::default())
        .add_materialized_view(Box::<NetworkPacketsMaterializedView>::default())
        .add_materialized_view(Box::<TotalHttpRequestsMaterializedView>::default())
    .build()
}