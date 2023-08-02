use sqlx::{Error, Postgres};
use sqlx::postgres::{PgQueryResult, PgRow};

const CHECK_EXISTENCE_QUERY: &str = "
    SELECT * FROM realtime_updating_history
    WHERE connection_id = $1;
";
const INSERT_CLIENT_QUERY: &str = "
    INSERT INTO realtime_updating_history (connection_id, last_used_index) VALUES ($1, $2);
";

const UPDATE_LAST_USED_INDEX_QUERY: &str = "
    UPDATE realtime_updating_history
    SET last_used_index = $1
    WHERE connection_id = $2;
";

const DELETE_CLIENT_QUERY: &str = "
    DELETE FROM realtime_updating_history
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