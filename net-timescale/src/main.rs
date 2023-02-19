use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use postgres::{Client, NoTls};

use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;
use net_timescale::command::dispatcher::CommandDispatcher;
use net_timescale::query::insert_packet::InsertPacket;

fn main() {
    thread::spawn(move || {
        let client = Client::connect("postgres://postgres:PsWDgxZb@localhost", NoTls).unwrap();

        let insert_packet = InsertPacket { client: Arc::new(Mutex::new(client)) };

        let queries = Arc::new(RwLock::new(HashMap::new()));
        queries.write().unwrap().insert("insert_packet".to_string(), insert_packet);

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
    }).join().unwrap();
}