use std::sync::Arc;

use host_core::connection_pool::configure_connection_pool;
use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_transport::quinn::connection::QuicConnection;
use sqlx::Pool;
use sqlx::Postgres;

use net_transport::quinn::server::builder::ServerQuicEndpointBuilder;

use crate::config::Config;
use crate::core::request_result::RequestResult;
use crate::core::update_manager::manager::UpdateManager;

pub struct Updater {
    config: Config,
    timesacledb_connection_pool: Arc<Pool<Postgres>>,
    #[allow(dead_code)]
    timescaledb_buffer_connection_pool: Arc<Pool<Postgres>>,
    update_manager: Arc<UpdateManager>,
}

impl Updater {
    pub async fn new(
        config: Config,
    ) -> Self {
        let timesacledb_connection_pool = Arc::new(
            configure_connection_pool(
                config.max_connection_size.size.parse().expect("not a number"),
                &config.timescaledb_connection_url.url,
            ).await
        );
        let timescaledb_buffer_connection_pool = Arc::new(
            configure_connection_pool(
                config.max_connection_size.size.parse().expect("not a number"),
                &config.timescaledb_buffer_connection_url.url,
            ).await
        );
        let update_manager = Arc::new(
            Updater::build_update_manager()
        );

        Self {
            timesacledb_connection_pool,
            timescaledb_buffer_connection_pool,
            config,
            update_manager
        }
    }

    fn build_update_manager() -> UpdateManager {
        UpdateManager::builder()
            .build()
    }

    pub async fn run(self) {
        log::info!("Run component");

        let config = Arc::new(self.config);

        log::info!("Run db migrations");
        let migrations_result = net_migrator::migrator::run_migrations(&self.timesacledb_connection_pool, "./migrations").await;
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
                    let handling_update_manager = self.update_manager.clone();
                    let handling_connection_pool = self.timesacledb_connection_pool.clone();
                    
                    tokio::spawn(async move {
                        Updater::handle_update_request(
                            client_connection,
                            handling_update_manager,
                            handling_connection_pool,
                        ).await
                    });
                },
                Err(_) => todo!(),
            }
        }
    }
    
    #[allow(unused_variables)]
    async fn handle_update_request(
        mut client_connection: QuicConnection,
        handling_update_manager: Arc<UpdateManager>,
        handling_connection_pool: Arc<Pool<Postgres>>,
    ) {
        let receive_result = client_connection.receive_reliable().await;
        if receive_result.is_err() {
            todo!()
        }
        let recieve_result = receive_result.unwrap();

        let enveloped_request = Envelope::decode(&recieve_result);

        let tenant_id = enveloped_request.get_tenant_id().to_owned();

        log::info!("Recieved request from client: {:?}", enveloped_request);

        let request_result = handling_update_manager.as_ref().handle_update(enveloped_request, handling_connection_pool).await;
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