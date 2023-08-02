use chrono::{DateTime, TimeZone, Utc};
use futures::executor::block_on;
use serde_json::json;
use sqlx::{Pool, Postgres};
use net_timescale_api::api::network_packet::NetworkPacketDTO;
use net_timescale::repository::network_packet;
use net_timescale::repository::network_packet::NetworkPacket;

async fn establish_connection() -> Pool<Postgres> {
    let database_url = "postgres://postgres:PsWDgxZb@localhost".to_owned();
    Pool::<Postgres>::connect("postgres://postgres:PsWDgxZb@localhost").await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(feature = "integration")]
#[test]
fn integration_test_insert() {
    let mut con = block_on(establish_connection());
    let json_data = json!({
        "test": "test",
    });
    let binary_json = serde_json::to_vec(&json_data).unwrap();
    let timestamp = 1688714981480935000;
    let network_packet_dto = NetworkPacketDTO::new(
        timestamp,
        "src",
        "dst",
        &binary_json,
    );
    let result = block_on(network_packet::insert_network_packet(
        &mut con, network_packet_dto.into()
    )).unwrap();
    assert_eq!(1, result.rows_affected());
    let query = sqlx::query_as::<_, NetworkPacket>("select * from captured_traffic;").fetch_all(&con);
    let query_result = block_on(query).unwrap();
    assert_eq!(query_result.len(), 1);
    let query_result = query_result.first().unwrap();
    assert_eq!(query_result.frame_time, Utc.timestamp_nanos(timestamp));
    assert_eq!(query_result.src_addr, "src");
    assert_eq!(query_result.dst_addr, "dst");
    assert_eq!(query_result.binary_data, json_data);
}