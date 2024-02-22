use std::sync::Arc;

use net_core_api::decoder_api::Decoder;
use net_core_api::encoder_api::Encoder;
use net_core_api::envelope::envelope::Envelope;
use net_transport::quinn::connection::QuicConnection;
use net_transport::quinn::server::builder::ServerQuicEndpointBuilder;
use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

use crate::config::Config;

use crate::continuous_aggregate::network_bandwidth_per_protocol::NetworkBandwidthPerProtocolAggregate;
use crate::continuous_aggregate::ContinuousAggregate;
use crate::continuous_aggregate::bandwidth_per_endpoint::BandwidthPerEndpointAggregate;
use crate::continuous_aggregate::network_bandwidth::NetworkBandwidthAggregate;
use crate::continuous_aggregate::network_graph::NetworkGraphAggregate;
use crate::continuous_aggregate::network_overview_filters::NetworkOverviewFiltersAggregate;

use crate::query::charts::bandwidth_per_endpoint::request::requester::NetworkBandwidthPerEndpointRequester;
use crate::query::charts::network_bandwidth::request::requester::NetworkBandwidthRequester;
use crate::query::charts::network_bandwidth_per_protocol::request::requester::NetworkBandwidthPerProtocolRequester;
use crate::query::charts::network_graph::request::requester::NetworkGraphRequester;
use crate::query::filters::network_overview::request::requester::NetworkOverviewFiltersRequester;
use crate::query::manager::query_manager::QueryManager; 


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

    fn build_query_manager() -> QueryManager {
        QueryManager::builder()
            .add_chart_generator(NetworkBandwidthPerEndpointRequester::default().boxed())
            .add_chart_generator(NetworkBandwidthRequester::default().boxed())
            .add_chart_generator(NetworkGraphRequester::default().boxed())
            .add_chart_generator(NetworkOverviewFiltersRequester::default().boxed())
            .build()
    }

    async fn create_continuous_aggregates(con: &Pool<Postgres>) {
        // TODO: refactor this part of code using, for example, continues aggregate manager
        // to reduce the amount of code here
        match NetworkGraphAggregate::create(con).await {
            Ok(_) => {
                log::info!("successfully created address pair continuous aggregate");
            },
            Err(err) => {
                log::debug!("couldn't create {}: {}", NetworkGraphAggregate::get_name(), err);
            }
        }
        match NetworkGraphAggregate::add_refresh_policy(con, None, None, "1 minute").await {
            Ok(_) => {
                log::info!("successfully created {} refresh policy", NetworkGraphAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {} refresh policy: {}", NetworkGraphAggregate::get_name(), err);
            }
        }
        match NetworkBandwidthAggregate::create(con).await {
            Ok(_) => {
                log::info!("successfully created {}", NetworkBandwidthAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {}: {}", NetworkBandwidthAggregate::get_name(), err);
            }
        }
        match NetworkBandwidthAggregate::add_refresh_policy(con, None, None, "1 minute").await {
            Ok(_) => {
                log::info!("successfully created {} refresh policy", NetworkBandwidthAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {} refresh policy: {}", NetworkBandwidthAggregate::get_name(), err);
            }
        }
        match BandwidthPerEndpointAggregate::create(con).await {
            Ok(_) => {
                log::info!("successfully created {}", BandwidthPerEndpointAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {}: {}", BandwidthPerEndpointAggregate::get_name(), err);
            }
        }
        match BandwidthPerEndpointAggregate::add_refresh_policy(con, None, None, "1 minute").await {
            Ok(_) => {
                log::info!("successfully created {} refresh policy", BandwidthPerEndpointAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {} refresh policy: {}", BandwidthPerEndpointAggregate::get_name(), err);
            }
        }
        match NetworkOverviewFiltersAggregate::create(con).await {
            Ok(_) => {
                log::info!("successfully created {}", NetworkOverviewFiltersAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {}: {}", NetworkOverviewFiltersAggregate::get_name(), err);
            }
        }
        match NetworkOverviewFiltersAggregate::add_refresh_policy(con, None, None, "1 minute").await {
            Ok(_) => {
                log::info!("successfully created {} refresh policy", NetworkOverviewFiltersAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {} refresh policy: {}", NetworkOverviewFiltersAggregate::get_name(), err);
            }
        }
        match NetworkBandwidthPerProtocolAggregate::create(con).await {
            Ok(_) => {
                log::info!("successfully created {}", NetworkBandwidthPerProtocolAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {}: {}", NetworkBandwidthPerProtocolAggregate::get_name(), err);
            }
        }
        match NetworkBandwidthPerProtocolAggregate::add_refresh_policy(con, None, None, "1 minute").await {
            Ok(_) => {
                log::info!("successfully created {} refresh policy", NetworkBandwidthPerProtocolAggregate::get_name());
            },
            Err(err) => {
                log::debug!("couldn't create {} refresh policy: {}", NetworkBandwidthPerProtocolAggregate::get_name(), err);
            }
        }
    }

    pub async fn run(self) {
        log::info!("Run component"); 
        Reporter::create_continuous_aggregates(&self.connection_pool).await;

        log::info!("Creating server endpoint for net-reporter..."); 
        let reporter_server_endpoint = ServerQuicEndpointBuilder::default()
            .with_addr(self.config.server_address.address.parse().unwrap())
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

        let recieve_enveloped_request = Envelope::decode(&recieve_result);

        log::info!("Recieved request from client: {:?}", recieve_enveloped_request);

        let response = query_manager.as_ref().handle_request(recieve_enveloped_request, connection_pool).await;
        if response.is_err() {
            todo!()
        }
        let response = response.unwrap();

        log::info!("Got response on request: {:?}", response);

        let send_result = client_connection.send_all_reliable(&response.encode()).await;
        if send_result.is_err() {
            todo!()
        }
    }
}