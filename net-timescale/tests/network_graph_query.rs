use chrono::Utc;
use futures::executor::block_on;
use sqlx::{Pool, Postgres};
use net_timescale::repository::realtime_client;
use net_timescale::persistence::network_graph;
use net_timescale::repository::network_packet;
use net_timescale::persistence::network_graph::NetworkGraphRequest;

async fn establish_connection() -> Pool<Postgres> {
    let database_url = "postgres://postgres:PsWDgxZb@localhost".to_owned();
    Pool::<Postgres>::connect("postgres://postgres:PsWDgxZb@localhost").await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[cfg(feature = "integration")]
#[test]
fn delete_present_client_by_handle_realtime() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 1;

    let mut con = block_on(establish_connection());
    let mut transaction: sqlx::Transaction<'_, Postgres> = block_on(con.begin()).unwrap();

    block_on(
        realtime_client::insert_client(
            &mut transaction,
            CONNECTION_ID,
            LAST_UPDATED_INDEX
        )
    ).unwrap();

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert_eq!(res.is_ok(), true);

    let ng_req = NetworkGraphRequest {
        start_date_time: Utc::now(),
        end_date_time: Utc::now(),
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

    assert_eq!(res.is_ok(), false);
}

#[cfg(feature = "integration")]
#[test]
fn update_present_client_by_handle_realtime() {
    const CONNECTION_ID: i64 = 1;
    const LAST_UPDATED_INDEX: i64 = 0;

    let mut con = block_on(establish_connection());
    let nt_packet = network_packet::NetworkPacket {
        frame_time: Utc::now(),
        src_addr: "".to_string(),
        dst_addr: "".to_string(),
        binary_data: serde_json::from_slice(&[]).unwrap(),
    };
    let res = block_on(
        network_packet::insert_network_packet(
            &con,
            nt_packet
        )
    );
    assert_eq!(res.is_ok(), res);

    let mut transaction: sqlx::Transaction<'_, Postgres> = block_on(con.begin()).unwrap();

    block_on(
        realtime_client::insert_client(
            &mut transaction,
            CONNECTION_ID,
            LAST_UPDATED_INDEX
        )
    ).unwrap();

    let res = block_on(realtime_client::check_client_id_existence(&mut transaction, CONNECTION_ID));

    assert_eq!(res.is_ok(), true);

    let ng_req = NetworkGraphRequest {
        start_date_time: Utc::now(),
        end_date_time: Utc::now(),
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

    assert_eq!(res.is_ok(), false);
    let index = res.unwrap().1;
    assert_eq!(1, index);
}

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
