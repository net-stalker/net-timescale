use chrono::{DateTime, Utc};
use futures::stream::BoxStream;
use sqlx::{Error, Pool, Postgres};

#[derive(sqlx::FromRow, Debug)]
pub struct AddressPair {
    pub src_addr: String,
    pub dst_addr: String,
}

const QUERY: &str = "
            SELECT src_addr, dst_addr
            FROM address_pair_aggregate
            WHERE bucket >= $1 AND bucket < $2
            GROUP BY src_addr, dst_addr
            ORDER BY src_addr, dst_addr;
        ";

pub async fn select_address_pairs_by_date_cut<'e>(
    con: &'e Pool<Postgres>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>
) -> BoxStream<'e, Result<AddressPair, Error>>
{
    sqlx::query_as::<_, AddressPair>(QUERY)
        .bind(start_date)
        .bind(end_date)
        .fetch(con)
}

pub async fn select_address_pairs_by_date_cut_transaction<'e>(
    transaction: &'e mut sqlx::Transaction<'_, Postgres>,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>
) -> Result<Vec<AddressPair>, Error>
{
    sqlx::query_as::<_, AddressPair>(QUERY)
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&mut **transaction)
        .await
}