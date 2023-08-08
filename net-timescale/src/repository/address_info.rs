use chrono::{DateTime, Utc};
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgConnection;
use futures::stream::BoxStream;


#[derive(sqlx::FromRow, Debug)]
pub struct AddressInfo {
    pub addr: String
    // may be expandable in future
}

const QUERY_BY_BUCKET: &str = "
    SELECT addr
    FROM (
        SELECT DISTINCT src_addr AS addr
        FROM address_pair_aggregate
        WHERE bucket >= $1 AND bucket < $2
        UNION
        SELECT distinct dst_addr as addr
        FROM address_pair_aggregate
        WHERE bucket >= $1 AND bucket < $2
    ) AS info
    ORDER BY addr;
";

pub async fn select_address_info_by_date_cut<'e>(
    con: &'e Pool<Postgres>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>
) -> BoxStream<'e, Result<AddressInfo, Error>>
{
    sqlx::query_as::<_, AddressInfo>(QUERY_BY_BUCKET)
        .bind(start_date)
        .bind(end_date)
        .fetch(con)
}

pub async fn select_address_info_by_date_cut_transaction<'e>(
    transaction: &'e mut sqlx::Transaction<'_, Postgres>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> Result<Vec<AddressInfo>, Error>
{
    sqlx::query_as::<_, AddressInfo>(QUERY_BY_BUCKET)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&mut **transaction)
        .await
}

const QUERY_BY_INDEX: &str = "
    SELECT addr
    FROM (
        SELECT DISTINCT(src_addr) AS addr
        FROM captured_traffic
        WHERE id >= $1
        UNION
        SELECT DISTINCT(dst_addr) AS addr
        FROM captured_traffic
        WHERE id >= $1
    ) AS info
    ORDER BY addr;
";

pub async fn select_address_info_by_index_transaction(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    index: i64
) -> Result<Vec<AddressInfo>, Error>
{
    sqlx::query_as::<_, AddressInfo>(QUERY_BY_INDEX)
        .bind(index)
        .fetch_all(&mut **transaction)
        .await
}