use chrono::{DateTime, Utc};
use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgConnection;
use futures::stream::BoxStream;


#[derive(sqlx::FromRow, Debug)]
pub struct AddressInfo {
    pub addr: String
    // may be expandable in future
}

pub async fn select_address_info_by_date_cut<'e>(
    con: &'e Pool<Postgres>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>
) -> BoxStream<'e, Result<AddressInfo, Error>>
{
    // TODO: this query isn't very efficient because we have to do 2 sub-queries.
    sqlx::query_as::<_, AddressInfo>(
        "
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
        "
    )
        .bind(start_date)
        .bind(end_date)
        .fetch(con)
}