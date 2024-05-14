use std::net::SocketAddr;
use std::sync::Arc;

use host_core::materialized_view::http_clients::HttpClientsMaterialiazedView;
use host_core::materialized_view::http_overview_filters::HttpOverviewFiltersMaterializedView;
use host_core::materialized_view::http_request_methods_distribution::HttpRequestMethodsDistributionMaterializedView;
use host_core::materialized_view::http_responses::HttpResponsesMaterializedView;
use host_core::materialized_view::http_responses_distribution::HttpResponsesDistributionMaterializedView;
use host_core::materialized_view::network_bandwidth::NetworkBandwidthMaterializedView;
use host_core::materialized_view::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointMaterializedView;
use host_core::materialized_view::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolMaterializedView;
use host_core::materialized_view::network_graph::NetworkGraphMaterializedView;
use host_core::materialized_view::network_overview_filters::NetworkOverviewFiltersMaterializedView;
use host_core::materialized_view::total_http_requests::TotalHttpRequestsMaterializedView;
use host_core::materialized_view::MaterializedView;
use net_component::component::network_service_component::NetworkServiceComponent;
use net_component::handler::network_service_handler_manager::NetworkServiceHandlerManager;
use net_component::handler::network_service_handler_manager_builder::NetworkServiceHandlerManagerBuilder;
use sqlx::Pool;
use sqlx::Postgres;

use crate::config::Config;
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
use crate::handlers::filters_handlers::network_overview::handler::NetworkOverviewFiltersHandler;
use host_core::connection_pool;

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
        // TODO: remove creation of materialized views out there
        create_materialized_view(&connection_pool).await;
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

async fn create_materialized_view(connection_pool: &Pool<Postgres>) {
    // TODO: refactor this part of code using, for example, continues aggregate manager
    // to reduce the amount of code here
    match HttpOverviewFiltersMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match HttpRequestMethodsDistributionMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match HttpResponsesMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match HttpClientsMaterialiazedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match HttpResponsesDistributionMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match NetworkBandwidthPerProtocolMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match TotalHttpRequestsMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match NetworkBandwidthPerEndpointMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match NetworkBandwidthMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match NetworkGraphMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
    match NetworkOverviewFiltersMaterializedView::create(connection_pool).await {
        Ok(_) => {
            // TODO: add logs
        },
        Err(err) => {
            log::debug!("{err}");
        }
    };
}
