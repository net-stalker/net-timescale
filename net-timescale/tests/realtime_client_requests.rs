use std::cmp::min;
use futures::executor::block_on;
use sqlx::{Pool, Postgres};
use net_timescale::repository::realtime_client;

async fn establish_connection() -> Pool<Postgres> {
    let database_url = "postgres://postgres:PsWDgxZb@localhost".to_owned();
    Pool::<Postgres>::connect("postgres://postgres:PsWDgxZb@localhost").await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[cfg(feature = "integration")]
#[test]
fn client_verify_present_test() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 1;

    let mut con = block_on(establish_connection());
    let mut transaction: sqlx::Transaction<'_, Postgres> = block_on(con.begin()).unwrap();


    block_on(sqlx::query(
        "insert into realtime_updating_history(connection_id, last_used_index) values($1, $2);"
    )
        .bind(CONNECTION_ID)
        .bind(LAST_UPDATED_INDEX)
        .execute(&mut *transaction)).unwrap();

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert_eq!(res.is_ok(), true);
}

#[cfg(feature = "integration")]
#[test]
fn client_verify_missing_test() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 1;

    let mut con = block_on(establish_connection());
    let mut transaction = block_on(con.begin()).unwrap();

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert_eq!(res.is_ok(), false);
}

#[cfg(feature = "integration")]
#[test]
fn client_insert_test() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 1;

    let mut con = block_on(establish_connection());
    let mut transaction = block_on(con.begin()).unwrap();
    let ans = block_on(realtime_client::insert_client(
        &mut transaction,
        CONNECTION_ID,
        LAST_UPDATED_INDEX)
    );
    assert_eq!(ans.is_ok(), true);
    assert_eq!(ans.unwrap().rows_affected(), 1);

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert_eq!(res.is_ok(), true);
}

#[cfg(feature = "integration")]
#[test]
fn client_delete_test() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 1;

    let mut con = block_on(establish_connection());
    let mut transaction = block_on(con.begin()).unwrap();

    let ans = block_on(realtime_client::insert_client(
        &mut transaction,
        CONNECTION_ID,
        LAST_UPDATED_INDEX)
    );

    assert_eq!(ans.is_ok(), true);
    assert_eq!(ans.unwrap().rows_affected(), 1);

    let ans = block_on(realtime_client::delete_client(
        &mut transaction,
        CONNECTION_ID
    ));

    assert_eq!(ans.is_ok(), true);
    assert_eq!(ans.unwrap().rows_affected(), 1);
}
#[cfg(feature = "integration")]
#[test]
fn get_min_index_test() {
    const CONNECTION_ID_1: i64 = 1;
    const LAST_UPDATED_INDEX_1: i64 = 1;

    const CONNECTION_ID_2: i64 = 2;
    const LAST_UPDATED_INDEX_2: i64 = 2;

    let expected_res: i64 = min(LAST_UPDATED_INDEX_1, LAST_UPDATED_INDEX_2);

    let con = block_on(establish_connection());
    let mut transaction = block_on(con.begin()).unwrap();
    let ans = block_on(realtime_client::insert_client(
        &mut transaction,
        CONNECTION_ID_1,
        LAST_UPDATED_INDEX_1)
    );
    assert_eq!(ans.is_ok(), true);
    assert_eq!(ans.unwrap().rows_affected(), 1);
    let ans = block_on(realtime_client::insert_client(
        &mut transaction,
        CONNECTION_ID_2,
        LAST_UPDATED_INDEX_2)
    );
    assert_eq!(ans.is_ok(), true);
    assert_eq!(ans.unwrap().rows_affected(), 1);

    let ans = block_on(realtime_client::get_min_index(
        &mut transaction
    ));

    assert!(ans.is_ok());
    let index = ans.unwrap();
    assert_eq!(index, expected_res);
}
