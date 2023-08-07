use sqlx::{Error, Postgres, Row};
use sqlx::postgres::{PgQueryResult, PgRow};

const CHECK_EXISTENCE_QUERY: &str = "
    SELECT * FROM realtime_updating_history
    WHERE connection_id = $1;
";
pub async fn check_client_id_existence(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    client_id: i64
) -> Result<PgRow, Error>
{
    sqlx::query(CHECK_EXISTENCE_QUERY)
        .bind(client_id)
        .fetch_one(&mut **transaction)
        .await
}

const INSERT_CLIENT_QUERY: &str = "
    INSERT INTO realtime_updating_history (connection_id, last_used_index) VALUES ($1, $2);
";
pub async fn insert_client(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    client_id: i64,
    last_updated_index: i64,
) -> Result<PgQueryResult, Error>
{
    sqlx::query(INSERT_CLIENT_QUERY)
        .bind(client_id)
        .bind(last_updated_index)
        .execute(&mut **transaction)
        .await
}

const UPDATE_LAST_USED_INDEX_QUERY: &str = "
    UPDATE realtime_updating_history
    SET last_used_index = $2
    WHERE connection_id = $1;
";
pub async fn update_last_index(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    client_id: i64,
    last_updated_index: i64,
) -> Result<PgQueryResult, Error>
{
    sqlx::query(UPDATE_LAST_USED_INDEX_QUERY)
        .bind(client_id)
        .bind(last_updated_index)
        .execute(&mut **transaction)
        .await
}

const DELETE_CLIENT_QUERY: &str = "
    DELETE FROM realtime_updating_history
    WHERE connection_id = $1;
";
pub async fn delete_client(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    client_id: i64,
) -> Result<PgQueryResult, Error>
{
    sqlx::query(DELETE_CLIENT_QUERY)
        .bind(client_id)
        .execute(&mut **transaction)
        .await
}
const GET_MIN_INDEX: &str = "
    SELECT MIN(last_used_index) AS index
    FROM realtime_updating_history;
";
pub async fn get_min_index(transaction: &mut sqlx::Transaction<'_, Postgres>) -> Result<i64, Error> {
    let res = sqlx::query(GET_MIN_INDEX)
        .fetch_one(&mut **transaction)
        .await;
    match res {
        Ok(row) => {
            let index: i64 = row.try_get("index").expect("index is expected to be queried");
            Ok(index)
        },
        Err(err) => {
            Err(err)
        }
    }
}
