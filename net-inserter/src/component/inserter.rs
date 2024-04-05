use std::sync::Arc;

use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

use net_agent_api::api::data_packet::DataPacketDTO;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::typed_api::Typed;
use net_core_api::core::decoder_api::Decoder;

use net_transport::quinn::connection::QuicConnection;
use net_transport::quinn::server::builder::ServerQuicEndpointBuilder;

use crate::config::Config;
use crate::utils::decoder;
use crate::utils::network_packet_inserter;

pub struct Inserter {
    config: Config,
    connection_pool: Arc<Pool<Postgres>>,
}

impl Inserter {
    pub async fn new(
        config: Config,
    ) -> Self {
        let connection_pool = Arc::new(
            Inserter::configure_connection_pool(&config).await
        );

        Self {
            connection_pool,
            config,
        }
    }

    async fn configure_connection_pool(config: &Config) -> Pool<Postgres> {
        PgPoolOptions::new()
            .max_connections(config.max_connection_size.size.parse().expect("not a number"))
            .connect(config.connection_url.url.as_str())
            .await
            .unwrap()
    }

    pub async fn handle_insert_request(
        pool: Arc<Pool<Postgres>>,
        mut client_connection: QuicConnection
    ) {
        let enveloped_request = match client_connection.receive_reliable().await {
            Ok(receive) => Envelope::decode(&receive),
            Err(_) => {
                log::error!("Error: Failed to receive request");
                return;
            },
        };

        let tenant_id = enveloped_request.get_tenant_id();

        if enveloped_request.get_type() != DataPacketDTO::get_data_type() {
            log::error!("Error: Request type is not DataPacketDTO");
            return;
        }

        let network_packet = match decoder::Decoder::decode(DataPacketDTO::decode(enveloped_request.get_data())).await {
            Ok(network_packet) => network_packet,
            Err(e) => {
                log::error!("{}", e);
                return;
            },
        };
        let mut transaction = pool.begin().await.unwrap();
        
        // TODO: later on it will be nice to open a stream of network packets and insert them in a batch
        match network_packet_inserter::insert_network_packet_transaction(
            &mut transaction, 
            tenant_id, 
            "MOCK_AGENT_ID", 
            &network_packet
        ).await {
            Ok(_) => log::info!("Successfully inserted network packet"),
            Err(e) => log::error!("Error: {}", e),
        }
        
        transaction.commit().await.unwrap();
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
                    let handling_connection_pool = self.connection_pool.clone();
                    tokio::spawn(async move {
                        Inserter::handle_insert_request(
                            handling_connection_pool,
                            client_connection
                        ).await
                    });
                },
                Err(_) => todo!(),
            }
        }
    }
}