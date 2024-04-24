use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

use net_transport::quinn::server::builder::ServerQuicEndpointBuilder;

use crate::config::Config;

pub struct Updater {
    config: Config,
    connection_pool: Arc<Pool<Postgres>>,
}

impl Updater {
    pub async fn new(
        config: Config,
    ) -> Self {
        let connection_pool = Arc::new(
            Updater::configure_connection_pool(&config).await
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
                    let config_clone = config.clone();
                    let handling_connection_pool = self.connection_pool.clone();
                    
                    tokio::spawn(async move {
                        Updater::handle_update_request(
                            config_clone,
                            handling_connection_pool,
                            client_connection,
                        ).await
                    });
                },
                Err(_) => todo!(),
            }
        }
    }
    
    #[allow(unused_variables)]
    async fn handle_update_request(
        config_clone: Arc<Config>,
        handling_connection_pool: Arc<Pool<Postgres>>,
        client_connection: net_transport::quinn::connection::QuicConnection
    ) {
        todo!()
    }
}