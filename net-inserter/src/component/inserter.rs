use std::sync::Arc;

use sqlx::Pool;
use sqlx::postgres::PgPoolOptions;
use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;

use net_transport::quinn::connection::QuicConnection;
use net_transport::quinn::server::builder::ServerQuicEndpointBuilder;

use crate::config::Config;
use crate::core::insert_handler::InsertHandler;

use super::dispatcher::Dispatcher;
use super::network::NetworkInserter;
use super::network::NetworkInserterCtor;
use super::pcap_file_inserter::PcapFileInserter;
use super::pcap_file_inserter::PcapFileInserterCtor;

pub struct Inserter {
    config: Config,
    connection_pool: Arc<Pool<Postgres>>,
    dispatcher: Arc<Dispatcher>,
}

impl Inserter {
    pub async fn new(
        config: Config,
    ) -> Self {
        let connection_pool = Arc::new(
            Inserter::configure_connection_pool(&config).await
        );
        let dispatcher = Arc::new(Self::configure_dispatcher(&config).await);

        Self {
            connection_pool,
            config,
            dispatcher,
        }
    }

    async fn configure_connection_pool(config: &Config) -> Pool<Postgres> {
        PgPoolOptions::new()
            .max_connections(config.max_connection_size.size.parse().expect("not a number"))
            .connect(config.connection_url.url.as_str())
            .await
            .unwrap()
    }

    async fn configure_dispatcher(config: &Config) -> Dispatcher {
        Dispatcher::default()
            .add_insertable(NetworkInserter::get_insertable_data_type(), Arc::new(NetworkInserterCtor::default()))
            .add_insertable(PcapFileInserter::get_insertable_data_type(), Arc::new(PcapFileInserterCtor::new(&config.pcaps.directory_to_save)))
    }

    pub async fn handle_insert_request(
        pool: Arc<Pool<Postgres>>,
        dispatcher: Arc<Dispatcher>,
        mut client_connection: QuicConnection
    ) {
        let enveloped_request = match client_connection.receive_reliable().await {
            Ok(receive) => Envelope::decode(&receive),
            Err(_) => {
                log::error!("Error: Failed to receive request");
                return;
            },
        };
        let envelope_type = enveloped_request.get_envelope_type().to_string();
        let insert_handler = dispatcher.get_insert_handler(enveloped_request.get_envelope_type());
        if insert_handler.is_none() {
            log::error!("Error: unknown data type to insert");
            return;
        }
        let insert_handler = insert_handler.unwrap();
        let mut transaction = pool.begin().await.unwrap();
        let res = insert_handler.insert(&mut transaction, enveloped_request).await;
        match res {
            Ok(_) => {
                log::debug!("{} is successfully inserted", envelope_type);
                let _ = transaction.commit().await;
            },
            Err(err) => log::error!("Error: {:?}", err)
        }
    }

    pub async fn run(self) {
        log::info!("Run component");

        let config = Arc::new(self.config);

        log::info!("Run db migrations");
        let migrations_result = net_migrator::migrator::run_migrations(&self.connection_pool, "./migrations").await;
        if migrations_result.is_err() {
            log::error!("Error, failed to run migrations: {}", migrations_result.err().unwrap());
            // TODO: Remove todo
            todo!();
        }
        log::info!("Successfully ran db migrations");

        log::info!("Creating server endpoint for net-reporter..."); 
        let reporter_server_endpoint = ServerQuicEndpointBuilder::default()
            .with_addr(config.server.addr.parse().unwrap())
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
                    let dispatcher_clone = self.dispatcher.clone();
                    tokio::spawn(async move {
                        Inserter::handle_insert_request(
                            handling_connection_pool,
                            dispatcher_clone,
                            client_connection,
                        ).await
                    });
                },
                Err(_) => todo!(),
            }
        }
    }
}