use diesel::{Connection, PgConnection, QueryableByName, RunQueryDsl, sql_query};
use diesel::sql_types::{
    Timestamptz,
    Text,
    Jsonb
};
use chrono::{DateTime, TimeZone, Utc};
use serde_json::json;
use net_core::jsons::json_parser::JsonParser;
use net_timescale_api::api::network_packet::NetworkPacketDTO;
use net_timescale::repository::network_packet;

fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres:PsWDgxZb@localhost".to_owned();
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
#[derive(QueryableByName, Debug)]
pub struct NetworkPacketTest {
    #[diesel(sql_type = Timestamptz)]
    frame_time: DateTime<Utc>,
    #[diesel(sql_type = Text)]
    src_addr: String,
    #[diesel(sql_type = Text)]
    dst_addr: String,
    #[diesel(sql_type = Jsonb)]
    binary_data: serde_json::Value,
}
#[cfg(feature = "integration")]
#[test]
fn integration_test_insert() {
    let mut con = establish_connection();
    let json_data = json!({
        "test": "test",
    });
    let binary_json = JsonParser::get_vec(json_data.clone());
    let timestamp = Utc.datetime_from_str("2020-01-01 08:56:00 +00:00", "%Y-%m-%d %H:%M:%S %z").unwrap();
    let timestamp = timestamp.timestamp_millis();
    let network_packet_dto = NetworkPacketDTO::new(
        timestamp,
        "src".to_string(),
        "dst".to_string(),
        binary_json.clone(),
    );
    let result = network_packet::insert_network_packet(
        &mut con, network_packet_dto.into()
    ).unwrap();
    assert_eq!(1, result);
    let query = sql_query("select * from captured_traffic;");
    let query_result: Vec<NetworkPacketTest> = query
        .load::<NetworkPacketTest>(&mut con)
        .unwrap();
    assert_eq!(query_result.len(), 1);
    let query_result = query_result.first().unwrap();
    assert_eq!(query_result.frame_time, Utc.timestamp_millis_opt(timestamp).unwrap());
    assert_eq!(query_result.src_addr, "src");
    assert_eq!(query_result.dst_addr, "dst");
    assert_eq!(query_result.binary_data, json_data);
}