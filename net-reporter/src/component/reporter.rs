use std::sync::Arc;

use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::typed_api::Typed;
use net_transport::quinn::connection::QuicConnection;
use net_transport::quinn::server::builder::ServerQuicEndpointBuilder;
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

use crate::config::Config;

use crate::materialized_view::MaterializedView;

use crate::materialized_view::http_overview_filters::HttpOverviewFiltersMaterializedView;
use crate::materialized_view::http_request_methods_distribution::HttpRequestMethodsDistributionMaterializedView;
use crate::materialized_view::http_responses::HttpResponsesMaterializedView;
use crate::materialized_view::http_clients::HttpClientsMaterialiazedView;
use crate::materialized_view::http_responses_distribution::HttpResponsesDistributionMaterializedView;
use crate::materialized_view::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolMaterializedView;
use crate::materialized_view::total_http_requests::TotalHttpRequestsMaterializedView;
use crate::materialized_view::network_bandwidth_per_endpoint::NetworkBandwidthPerEndpointMaterializedView;
use crate::materialized_view::network_bandwidth::NetworkBandwidthMaterializedView;
use crate::materialized_view::network_graph::NetworkGraphMaterializedView;
use crate::materialized_view::network_overview_filters::NetworkOverviewFiltersMaterializedView;

use crate::query::charts::network_bandwidth_per_endpoint::request::requester::NetworkBandwidthPerEndpointRequester;
use crate::query::charts::http_request_methods_distribution::request::requester::HttpRequestMethodsDistributionRequester;
use crate::query::charts::http_responses::request::requester::HttpResponsesRequester;
use crate::query::charts::http_clients::request::requester::HttpClientsRequester;
use crate::query::charts::http_responses_distribution::request::requester::HttpResponsesDistributionRequester;
use crate::query::charts::network_bandwidth::request::requester::NetworkBandwidthRequester;
use crate::query::charts::network_bandwidth_per_protocol::request::requester::NetworkBandwidthPerProtocolRequester;
use crate::query::charts::network_graph::request::requester::NetworkGraphRequester;
use crate::query::charts::total_http_requests::request::requester::TotalHttpRequestsRequester;
use crate::query::filters::http_overview::request::requester::HttpOverviewFiltersRequester;
use crate::query::filters::network_overview::request::requester::NetworkOverviewFiltersRequester;
use crate::query::manager::query_manager::QueryManager;
use crate::query::request_result::RequestResult; 


pub struct Reporter {
    config: Config,
    connection_pool: Arc<Pool<Postgres>>,
    query_manager: Arc<QueryManager>,
}

impl Reporter {
    pub async fn new(
        config: Config
    ) -> Self {
        let connection_pool = Arc::new(
            Reporter::configure_connection_pool(&config).await
        );
        let query_manager = Arc::new(
            Reporter::build_query_manager()
        );

        Self {
            connection_pool,
            config,
            query_manager,
        }
    }

    async fn configure_connection_pool(config: &Config) -> Pool<Postgres> {
        PgPoolOptions::new()
            .max_connections(config.max_connection_size.size.parse().expect("not a number"))
            .connect(config.connection_url.url.as_str())
            .await
            .unwrap()
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

    fn build_query_manager() -> QueryManager {
        QueryManager::builder()
            .add_chart_generator(NetworkBandwidthPerEndpointRequester::default().boxed())
            .add_chart_generator(NetworkBandwidthPerProtocolRequester::default().boxed())
            .add_chart_generator(NetworkBandwidthRequester::default().boxed())
            .add_chart_generator(NetworkGraphRequester::default().boxed())
            .add_chart_generator(NetworkOverviewFiltersRequester::default().boxed())
            .add_chart_generator(TotalHttpRequestsRequester::default().boxed())
            .add_chart_generator(HttpRequestMethodsDistributionRequester::default().boxed())
            .add_chart_generator(HttpResponsesRequester::default().boxed())
            .add_chart_generator(HttpClientsRequester::default().boxed())
            .add_chart_generator(HttpResponsesDistributionRequester::default().boxed())
            .add_chart_generator(HttpOverviewFiltersRequester::default().boxed())
            .build()
    }

    pub async fn run(self) {
        log::info!("Run component"); 

        log::info!("Run db migrations");
        let migrations_result = net_migrator::migrator::run_migrations(&self.connection_pool, "./migrations").await;
        if migrations_result.is_err() {
            log::error!("Error, failed to run migrations: {}", migrations_result.err().unwrap());
            todo!();
        }
        log::info!("Successfully ran db migrations");

        Reporter::create_materialized_view(&self.connection_pool).await;

        log::info!("Creating server endpoint for net-reporter..."); 
        
        let reporter_server_endpoint = ServerQuicEndpointBuilder::default()
            .with_addr(self.config.server.addr.parse().unwrap())
            .build();
        if reporter_server_endpoint.is_err() {
            todo!()
        }
        let mut reporter_server_endpoint = reporter_server_endpoint.unwrap();
        log::info!("Successfully created server endpoint for net-reporter");

        loop {
            log::info!("Waiting on client connection...");
            let client_connection_result = reporter_server_endpoint.accept_client_connection().await;
            match client_connection_result {
                Ok(client_connection) => {
                    log::info!("Client is successfully connected");
                    let handling_query_manager = self.query_manager.clone();
                    let handling_connection_pool = self.connection_pool.clone();
                    tokio::spawn(async move {
                        Reporter::handle_client_connection(
                            client_connection,
                            handling_query_manager,
                            handling_connection_pool
                        ).await
                    });
                },
                Err(_) => todo!(),
            }
        }
    }

    //TODO: Write error handling for receiving and sending result errors.
    async fn handle_client_connection(
        mut client_connection: QuicConnection,
        query_manager: Arc<QueryManager>,
        connection_pool: Arc<Pool<Postgres>>
    ) {
        let receive_result = client_connection.receive_reliable().await;
        if receive_result.is_err() {
            todo!()
        }
        let recieve_result = receive_result.unwrap();

        let enveloped_request = Envelope::decode(&recieve_result);

        let tenant_id = enveloped_request.get_tenant_id().to_owned();

        log::info!("Recieved request from client: {:?}", enveloped_request);

        let request_result = query_manager.as_ref().handle_request(enveloped_request, connection_pool).await;
        log::info!("Got response on request: {:?}", request_result);

        let request_result: RequestResult = request_result.into();
        let request_result_dto: ResultDTO = request_result.into();
        let envelope_to_send = Envelope::new(
            &tenant_id,
            ResultDTO::get_data_type(),
            &request_result_dto.encode()
        );

        let send_result = client_connection.send_all_reliable(&envelope_to_send.encode()).await;
        if send_result.is_err() {
            todo!()
        }
    }
}