use std::net::IpAddr;
use std::path::Path;
use tokio::fs;

pub async fn get_addr_for_host(host_name: &str) -> String {
    let hosts_file = Path::new("/etc/hosts");
    let contents = fs::read_to_string(hosts_file).await.expect("Failed to read /etc/hosts");

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(ip_addr) = parts[0].parse::<IpAddr>() {
                if parts[1] == host_name {
                    return ip_addr.to_string();
                }
            }
        }
    }
    panic!("Failed to find ip address for host name: {}", host_name);
}

pub mod materialized_view;
pub mod pcaps;
pub mod connection_pool;
