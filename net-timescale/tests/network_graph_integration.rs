mod connection_establishing;

use async_std::task::block_on;
use connection_establishing::establish_connection;
#[test]
#[ignore]
fn test_creating_network_graph() {
    block_on(establish_connection());
    assert!(true);
}