use std::net::SocketAddr;
use std::sync::Arc;

use net_component::component::network_service_component::NetworkServiceComponent;
use net_component::handler::network_service_handler_manager::NetworkServiceHandlerManager;
use net_component::handler::network_service_handler_manager_builder::NetworkServiceHandlerManagerBuilder;
use sqlx::Pool;
use sqlx::Postgres;

use crate::config::Config;
use crate::handlers::network_insert_handler::InsertNetworkHandler;
use crate::handlers::pcap_file_insert_handler::InsertPcapFileHandler;
use component_core::connection_pool;

pub struct InserterComponent {
    connection_pool: Arc<Pool<Postgres>>,
    server_addr: SocketAddr,
    handling_manager: Arc<NetworkServiceHandlerManager>,
}

impl InserterComponent {
    pub async fn new(config: &Config) -> Self {
        let connection_pool = Arc::new(
            connection_pool::configure_connection_pool(
                &config.connection_url.url,
                config.max_connection_size.size.parse().expect("Valid number of max connection size is expected")
            ).await
        );
        let server_addr: SocketAddr = config.server.addr.parse().expect("Valid server address is expected");
        let handling_manager = Self::build_handling_manager(config).await;

        Self {
            connection_pool,
            server_addr,
            handling_manager,
        }
    }

    async fn build_handling_manager(config: &Config) -> Arc<NetworkServiceHandlerManager> {
        Arc::new(
            NetworkServiceHandlerManagerBuilder::default()
                .add_handler(InsertNetworkHandler::default().boxed())
                .add_handler(InsertPcapFileHandler::new(&config.pcaps.directory_to_save).boxed())
                .build()
        )
    }
}

#[async_trait::async_trait]
impl NetworkServiceComponent for InserterComponent {
    fn get_connection_pool(&self) -> Arc<Pool<Postgres> >  {
        self.connection_pool.clone()
    }

    fn get_server_addr(&self) -> SocketAddr {
        self.server_addr
    }

    fn get_handling_manager(&self) -> Arc<NetworkServiceHandlerManager>  {
        self.handling_manager.clone()
    }
}
