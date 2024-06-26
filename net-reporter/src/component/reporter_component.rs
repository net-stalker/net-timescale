use std::net::SocketAddr;
use std::sync::Arc;

use net_component::component::network_service_component::NetworkServiceComponent;
use net_component::handler::network_service_handler_manager::NetworkServiceHandlerManager;
use net_component::handler::network_service_handler_manager_builder::NetworkServiceHandlerManagerBuilder;

use sqlx::Pool;
use sqlx::Postgres;

use crate::config::Config;

use crate::handlers::buffer_handlers::buffer::handler::BufferHandler;
use crate::handlers::chart_handlers::http_clients::handler::HttpClientsHandler;
use crate::handlers::chart_handlers::http_request_methods_distribution::handler::HttpRequestMethodsDistributionHandler;
use crate::handlers::chart_handlers::http_responses::handler::HttpResponsesHandler;
use crate::handlers::chart_handlers::http_responses_distribution::handler::HttpResponsesDistributionHandler;
use crate::handlers::chart_handlers::network_bandwidth::handler::NetworkBandwidthHandler;
use crate::handlers::chart_handlers::network_bandwidth_per_endpoint::handler::NetworkBandwidthPerEndpointHandler;
use crate::handlers::chart_handlers::network_bandwidth_per_protocol::handler::NetworkBandwidthPerProtocolHandler;
use crate::handlers::chart_handlers::network_graph::handle::handler::NetworkGraphHandler;
use crate::handlers::chart_handlers::total_http_requests::handler::TotalHttpRequestsHandler;
use crate::handlers::filters_handlers::http_overview::handle::handler::HttpOverviewFiltersHandler;
use crate::handlers::filters_handlers::network_overview::handle::handler::NetworkOverviewFiltersHandler;
use crate::handlers::network_handlers::network_id::handler::NetworkIdHandler;
use crate::handlers::network_handlers::networks::handler::NetworksHandler;
use crate::handlers::network_packet_handlers::network_packets::handler::NetworkPacketsHandler;

use component_core::connection_pool;

pub struct ReporterComponent {
    connection_pool: Arc<Pool<Postgres>>,
    server_addr: SocketAddr,
    handling_manager: Arc<NetworkServiceHandlerManager>,
}

impl ReporterComponent {
    pub async fn new(config: &Config) -> Self {
        let connection_pool = Arc::new(
            connection_pool::configure_connection_pool(
                &config.connection_url.url,
                config.max_connection_size.size.parse().expect("Valid number of max connection size is expected")
            ).await
        );
        let server_addr: SocketAddr = config.server.addr.parse().expect("Valid server address is expected");
        let handling_manager = Self::build_handling_manager().await;

        Self {
            connection_pool,
            server_addr,
            handling_manager,
        }
    }

    async fn build_handling_manager() -> Arc<NetworkServiceHandlerManager> {
        Arc::new(
            NetworkServiceHandlerManagerBuilder::default()
                .add_handler(HttpClientsHandler::default().boxed())
                .add_handler(HttpRequestMethodsDistributionHandler::default().boxed())
                .add_handler(HttpResponsesHandler::default().boxed())
                .add_handler(HttpResponsesDistributionHandler::default().boxed())
                .add_handler(NetworkBandwidthHandler::default().boxed())
                .add_handler(NetworkBandwidthPerEndpointHandler::default().boxed())
                .add_handler(NetworkBandwidthPerProtocolHandler::default().boxed())
                .add_handler(NetworkGraphHandler::default().boxed())
                .add_handler(TotalHttpRequestsHandler::default().boxed())
                .add_handler(HttpOverviewFiltersHandler::default().boxed())
                .add_handler(NetworkOverviewFiltersHandler::default().boxed())
                .add_handler(NetworkIdHandler::default().boxed())
                .add_handler(NetworksHandler::default().boxed())
                .add_handler(NetworkPacketsHandler::default().boxed())
                .add_handler(BufferHandler::default().boxed())
                .build()
        )
    }
}

#[async_trait::async_trait]
impl NetworkServiceComponent for ReporterComponent {
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
