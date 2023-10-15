mod connection_establishing;
mod continuous_aggregate;

use async_std::task::block_on;
use chrono::NaiveDateTime;
use sqlx::{Pool, Postgres};
use connection_establishing::establish_connection;
use continuous_aggregate::*;
async fn set_test_data_set_for_graph(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
) {
    // need to add some inserts in bd for test graph
    // 1) need to create 2 tests at least
    // 2) Need to test with NetworkGraphDTO
    // 3) 1 test simpe, 3 nodes, 3 links
    // 4) 2 test more complex, 5 nodes, 6 links
    // '2020-01-01 00:00:00 +00:00' 0, 0, 1, 2, json::{"l1: {frame: frame.protocols}"}
    let time1 = NaiveDateTime::parse_from_str("Sat, 11 Feb 2023 23:40:00.000000000 UTC", "%a, %d %b %Y %H:%M:%S.%f %Z").unwrap().and_utc();
}

struct TestContext {
    pub con: Pool<Postgres>,
}

impl Drop for TestContext {
    fn drop(&mut self) {
        println!("Test teardown ...");
        block_on(drop_data_aggregate(&self.con)).unwrap();
        block_on(sqlx::query(
            "delete from captured_traffic;"
        ).execute(&self.con))
            .unwrap();
    }
}

impl TestContext{
    fn setup() -> TestContext {
        println!("Test setup ...");
        let con = block_on(establish_connection());
        TestContext {
            con
        }
    }
}
#[test]
#[ignore]
fn test_creating_network_graph() {
    let con = TestContext::setup();
    // TODO: fill the table
    assert!(true);
}