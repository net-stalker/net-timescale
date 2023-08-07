use chrono::{DateTime, Utc};
use sqlx::{Error, Pool, Postgres, Row};
use sqlx::postgres::{PgConnection, PgRow};
use futures::stream::BoxStream;


const GET_LAST_PACKET_ID_QUERY: &str = "
    SELECT MAX(id)
    FROM captured_traffic;
";

pub async fn get_max_id(
    transaction: &mut sqlx::Transaction<'_, Postgres>
) -> Result<i32, Error>
{
    let res = sqlx::query(GET_LAST_PACKET_ID_QUERY)
        .fetch_one(&mut **transaction)
        .await;
    match res {
        Ok(row) => {
            Ok(row.get(0))
        },
        Err(err) => {
            Err(err)
        }
    }
}
