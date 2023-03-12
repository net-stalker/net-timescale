use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;

use log::info;
use postgres::{Client, NoTls};
use shaku::{module, Component};

use net_core::starter::starter::Starter;
use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;

use crate::command::dispatcher::CommandDispatcher;
use crate::query::insert_packet::InsertPacket;
use crate::query::query_packet::QueryPacket;

module! {
    pub NetTimescaleModule {
        components = [Timescale],
        providers = []
    }
}

#[derive(Component)]
#[shaku(interface = Starter)]
pub struct Timescale;

impl Starter for Timescale {
    fn start(&self) -> JoinHandle<()> {
        thread::spawn(move || {
            info!("Start module");

            let client = Client::connect("postgres://postgres:PsWDgxZb@localhost", NoTls).unwrap();

            let insert_packet = InsertPacket {
                client: Arc::new(Mutex::new(client)),
            };

            let queries = Arc::new(RwLock::new(HashMap::new()));
            queries
                .write()
                .unwrap()
                .insert("insert_packet".to_string(), insert_packet);

            //TODO should use pool of connections
            let client = Client::connect("postgres://postgres:PsWDgxZb@localhost", NoTls).unwrap();
            let packet = QueryPacket {
                client: Arc::new(Mutex::new(client)),
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
        })
    }
}