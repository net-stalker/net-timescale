use chrono::{DateTime, Utc};
use net_timescale_api::api::bandwidth_per_endpoint::endpoint::EndpointDTO;

use sqlx::{Error, Pool, Postgres, Transaction};
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Endpoint {
    id: String,
    bytes_sent: Option<i64>,
    bytes_received: Option<i64>,
}

impl From<Endpoint> for EndpointDTO {
    fn from(value: Endpoint) -> Self {
        EndpointDTO::new(
            value.id.as_str(),
            value.bytes_received.unwrap_or(0),
            value.bytes_sent.unwrap_or(0),
        )
    }
}

const SELECT_BY_DATE_CUT: &str = "
        select
            COALESCE(lhs.id, rhs.id) as id,
            lhs.bytes_sent as bytes_sent,
            rhs.bytes_received as bytes_received
        from
            (
                select
                    src_addr as id,
                    SUM(packet_length) as bytes_sent
                from bandwidth_per_endpoint_aggregate
                where group_id = $1 AND bucket >= $2 AND bucket < $3
                group by src_addr
            ) as lhs full outer join (
                select
                    dst_addr as id,
                    SUM(packet_length) as bytes_received
                from bandwidth_per_endpoint_aggregate
                where group_id = $1 AND bucket >= $2 AND bucket < $3
                group by dst_addr
            ) as rhs on lhs.id = rhs.id;
";

impl Endpoint {
    pub async fn select_by_date_cut(
        con: &Pool<Postgres>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Endpoint>, Error>
    {
        sqlx::query_as(SELECT_BY_DATE_CUT)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(con)
            .await
    }
    pub async fn transaction_select_by_date_cut(
        transaction: &mut Transaction<'_, Postgres>,
        group_id: Option<&str>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Endpoint>, Error>
    {
        sqlx::query_as(SELECT_BY_DATE_CUT)
            .bind(group_id)
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&mut **transaction)
            .await
    }
}
