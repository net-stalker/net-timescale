use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use log::info;
use postgres::{NoTls};
use threadpool::ThreadPool;
use net_core::layer::NetComponent;
use r2d2_postgres::{PostgresConnectionManager};

use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;

use crate::command::dispatcher::CommandDispatcher;
use crate::query::insert_packet::InsertPacket;
use crate::query::query_packet::QueryPacket;

pub struct Timescale {
    pub pool: ThreadPool,
}

impl Timescale {
    pub fn new(pool: ThreadPool) -> Self {
        Self { pool }
    }
}

impl NetComponent for Timescale {
    fn run(self) {
        self.pool.execute(move || {
            info!("Run component");
            let manager = PostgresConnectionManager::new(
                "postgres://postgres:PsWDgxZb@localhost".parse().unwrap(),
                NoTls
            );
            let pool = r2d2::Pool::builder().max_size(2).build(manager).unwrap();

            let insert_packet = InsertPacket {
                pool: Arc::new(Mutex::new(pool.clone())),
            };
            let queries = Arc::new(RwLock::new(HashMap::new()));
            queries
                .write()
                .unwrap()
                .insert("insert_packet".to_string(), insert_packet);

            let packet = QueryPacket {
                pool: Arc::new(Mutex::new(pool.clone())),
            };

            packet.subscribe();

            let command_dispatcher = CommandDispatcher { queries };

            let db_service = ConnectorNNG::builder()
                .with_endpoint("tcp://0.0.0.0:5556".to_string())
                .with_proto(Proto::Rep)
                .with_handler(command_dispatcher)
                .build()
                .bind()
                .into_inner();

            Poller::new()
                .add(db_service)
                .poll();
        });
    }
}