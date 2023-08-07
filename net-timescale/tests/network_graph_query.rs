use chrono::{TimeZone, Utc};
use futures::executor::block_on;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use net_timescale::repository::realtime_client;
use net_timescale::persistence::network_graph;
use net_timescale::repository::network_packet;
use net_timescale::persistence::network_graph::NetworkGraphRequest;

async fn establish_connection() -> Pool<Postgres> {
    let database_url = "postgres://postgres:PsWDgxZb@localhost".to_owned();
    Pool::<Postgres>::connect("postgres://postgres:PsWDgxZb@localhost").await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn insert_tests_packet(
    transaction: &mut sqlx::Transaction<'_, Postgres>
)  {
    let nt_packet = network_packet::NetworkPacket {
        frame_time: Utc::now(),
        src_addr: "".to_string(),
        dst_addr: "".to_string(),
        binary_data: json!({"test": "test",}),
    };
    let res = network_packet::insert_network_packet_transaction(
        transaction,
        nt_packet
    ).await;
    assert!(res.is_ok());
}

#[cfg(feature = "integration")]
#[test]
fn delete_present_client_by_handle_realtime() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 1;

    let mut con = block_on(establish_connection());
    let mut transaction: sqlx::Transaction<'_, Postgres> = block_on(con.begin()).unwrap();

    // block_on(insert_tests_packet(&mut transaction));

    block_on(
        realtime_client::insert_client(
            &mut transaction,
            CONNECTION_ID,
            LAST_UPDATED_INDEX
        )
    ).unwrap();

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert!(res.is_ok());

    let ng_req = NetworkGraphRequest {
        start_date_time: Utc::now(),
        end_date_time: Utc.timestamp_nanos(0),
        is_subscribe: false
    };


    block_on(
        network_graph::handle_realtime_request(
            &mut transaction,
            &ng_req,
            CONNECTION_ID
        )
    );


    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert!(res.is_err());
}

#[cfg(feature = "integration")]
#[test]
fn update_present_client_by_handle_realtime() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 0;

    let mut con = block_on(establish_connection());
    let mut transaction: sqlx::Transaction<'_, Postgres> = block_on(con.begin()).unwrap();

    block_on(insert_tests_packet(&mut transaction));

    block_on(
        realtime_client::insert_client(
            &mut transaction,
            CONNECTION_ID,
            LAST_UPDATED_INDEX
        )
    ).unwrap();

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert!(res.is_ok());

    let ng_req = NetworkGraphRequest {
        start_date_time: Utc::now(),
        end_date_time: Utc.timestamp_nanos(0),
        is_subscribe: true
    };
    let test_date = ng_req.end_date_time.timestamp_nanos();

    block_on(
        network_graph::handle_realtime_request(
            &mut transaction,
            &ng_req,
            CONNECTION_ID
        )
    );


    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert!(res.is_ok());
    let new_index: i64 = res.unwrap().try_get("last_used_index").unwrap();
    assert_ne!(LAST_UPDATED_INDEX, new_index);
}

#[cfg(feature = "integration")]
#[test]
fn client_insert_by_handle_realtime() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 1;

    let mut con = block_on(establish_connection());
    let mut transaction: sqlx::Transaction<'_, Postgres> = block_on(con.begin()).unwrap();

    block_on(insert_tests_packet(&mut transaction));

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert!(res.is_err());

    let ng_req = NetworkGraphRequest {
        start_date_time: Utc::now(),
        end_date_time: Utc.timestamp_nanos(0),
        is_subscribe: true
    };


    block_on(
        network_graph::handle_realtime_request(
            &mut transaction,
            &ng_req,
            CONNECTION_ID
        )
    );

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert!(res.is_ok());
}

#[cfg(feature = "integration")]
#[test]
fn client_delete_by_handle_realtime() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 1;

    let mut con = block_on(establish_connection());
    let mut transaction: sqlx::Transaction<'_, Postgres> = block_on(con.begin()).unwrap();

    assert!(
        block_on(realtime_client::insert_client(&mut transaction, CONNECTION_ID, LAST_UPDATED_INDEX)).is_ok()
    );

    let ng_req = NetworkGraphRequest {
        start_date_time: Utc::now(),
        end_date_time: Utc.timestamp_nanos(0),
        is_subscribe: false,
    };


    block_on(
        network_graph::handle_realtime_request(
            &mut transaction,
            &ng_req,
            CONNECTION_ID
        )
    );

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert!(res.is_err());
}
