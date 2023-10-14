use chrono::{DateTime, Utc};

use sqlx::{Error, Postgres, Transaction};
#[derive(sqlx::FromRow, Debug)]
struct Endpoint {
    id: String,
    bytes_sent: i64,
    bytes_received: i64,
}

impl Endpoint {
    pub async fn select_by_date_cut (
        transaction: &mut Transaction<'_, Postgres>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Endpoint>, Error>
    {
        sqlx::query_as("
            select COALESCE(lhs.id, rhs.id) as id, lhs.bytes_sent as bytes_sent, rhs.bytes_received as bytes_received
            from (
                select src_addr as id, SUM(packet_length) as bytes_sent
                from data_aggregate
                where bucket >= $1 AND bucket < $2
                group by src_addr
            ) as lhs
            full outer join (
                select dst_addr as id, SUM(packet_length) as bytes_received
                from data_aggregate
                where bucket >= $1 AND bucket < $2
                group by dst_addr
            ) as rhs
            on lhs.id = rhs.id;
        ")
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&mut **transaction)
            .await
    }
}