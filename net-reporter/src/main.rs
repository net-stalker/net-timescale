use net_reporter::config::Config;
use net_reporter::component::reporter::Reporter;
use passivized_docker_engine_client::requests::{CreateContainerRequest, Filters, ListContainersRequest};
use passivized_docker_engine_client::DockerEngineClient;

#[tokio::main]
async fn main() {
    init_log();
    log::info!("Run module");
    let config = if cfg!(debug_assertions) {
        println!("Running in debug mode");
        // return config
        Config::builder().build().expect("read config error")
    } else {
        println!("Running in release mode");
        // return config
        Config {
            server: net_reporter::config::Server {
                host_name: "net-reporter".to_string(),
                port: 8080,
            },
            connection_url: net_reporter::config::ConnectionUrl {
                url: "postgres://postgres:PsWDgxZb@localhost:5433/?sslmode=require&sslcert=docker/.ssl/client.crt&sslkey=docker/.ssl/client.key".to_string(),
            },
            max_connection_size: net_reporter::config::MaxConnectionSize {
                size: "10".to_string(),
            },
            fusion_auth_server_addres: net_reporter::config::FusionAuthServerAddres {
                addr: "http://localhost:8080".to_string(),
            },
            fusion_auth_api_key: net_reporter::config::FusionAuthApiKey {
                key: "gw_awXW6h11DZ7ncwyim23-wQ76IAsM947L5p9Wb7yYOR0Erh_yQKCD4".to_string(),
            },
        }
    };
    // let host_name = "docker-timescaledb".to_string();
    // let client = DockerEngineClient::new().unwrap();
    // let containers = client.containers();
    // for container in containers.list(ListContainersRequest {
    //     all: Some(true),
    //     limit: None,
    //     size: Some(true),
    //     filters: Filters::default(),
    
    // }).await.unwrap() {
    //     log::info!("container : {:?}", container);
    //     if container.image == host_name {
    //         log::info!("possble ip address {:?}", container.network_settings.first_ip_address());
    //     }
    // }
    
    let reporter_component = Reporter::new(config).await;
    
    log::info!("Created component");
    
    reporter_component.run().await;
}

fn init_log() {
    let config_str = include_str!("log4rs.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();
}