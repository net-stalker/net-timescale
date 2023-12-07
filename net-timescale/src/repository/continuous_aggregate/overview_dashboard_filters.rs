// TODO: as for now it will be named overview_dashboard_filters. But from the future perspective
// it makes sense to name it overview_dashboard_essentials and have a single CA for a dashboard
// it will also impact overall logic of querying dashboard
// consider the following flow:
// 1) receive dashboard request from REST server which isn't implemented at the moment
// 2) Query all the necessary data from CA from time to time
// 3) Send it back using to REST server (at this point REST server is like a proxy)
// 4) REST server will parse the received data and form dashboard entity (because there it's aware of business logic)
// 5) REST client will only receive JSON which will easily parsed and used for visualisation

use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgQueryResult;
use crate::repository::continuous_aggregate::ContinuousAggregate;

pub struct OverviewDashboardFiltersAggregate { }

const CA_NAME: &str = "overview_dashboard_filters";

#[async_trait::async_trait]
impl ContinuousAggregate for OverviewDashboardFiltersAggregate {
    fn get_name() -> &'static str {
        CA_NAME
    }

    async fn create(pool: &Pool<Postgres>) -> Result<PgQueryResult, Error> {
        let query = format!(
            "
                CREATE MATERIALIZED VIEW {}
                WITH (timescaledb.continuous) AS
                SELECT
                    time_bucket('2 minutes', frame_time) AS bucket,
                    group_id,
                    agent_id,
                    (binary_data->'l1'->'frame'->>'frame.len')::integer AS packet_length
                FROM captured_traffic
                GROUP BY bucket, group_id, agent_id, packet_length;
            ",
            Self::get_name()
        );
        sqlx::query(query.as_str())
            .execute(pool)
            .await
    }
}