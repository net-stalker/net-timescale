use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use log::{debug, info};
use simple_websockets::Event;
use threadpool::ThreadPool;
use net_core::{layer::NetComponent, transport::{sockets::Sender, dummy_command::DummyCommand}};

use net_core::transport::{
    connector_nng_pub_sub::ConnectorNNGPubSub,
    connector_nng::{ConnectorNNG, Proto}
};
use net_core::transport::polling::nng::NngPoller;

use crate::command::{agent::AgentCommand, dummy_timescale::DummyTimescaleHandler};
use crate::command::pull::PullCommand;
use crate::command::translator::TranslatorCommand;

pub struct Hub {
    pub pool: ThreadPool,
}

impl Hub {
    pub fn new(pool: ThreadPool) -> Self {
        Hub { pool }
    }
}
impl NetComponent for Hub {
    fn run(self) {
        info!("run component");

        let clients = Arc::new(RwLock::new(HashMap::new()));
        let clients_inner = clients.clone();

        //TODO use instead ws from ConnectoNNG
        self.pool.execute(move || {
            let event_hub = simple_websockets::launch(9091)
                .expect("failed to listen on port 9091");

            loop {
                match event_hub.poll_event() {
                    Event::Connect(client_id, responder) => {
                        info!("A client connected with id #{}", client_id);
                        clients_inner
                            .write()
                            .unwrap()
                            .insert(client_id, responder.clone());

                        //TODO for every websocket conn should be created new zmq socket referenced to the websocket client connection.
                    }
                    Event::Disconnect(client_id) => {
                        info!("Client #{} disconnected.", client_id);
                        clients_inner.write().unwrap().remove(&client_id);
                    }
                    Event::Message(client_id, message) => {
                        debug!(
                            "Received a message from client #{}: {:?}",
                            client_id, message
                        );
                        // let responder = clients.get(&client_id).unwrap();
                        // let responder = clients.write().unwrap().get(&client_id).unwrap();
                        // responder.send(message);
                    }
                }
            }
        });
        self.pool.execute(move || {
            // TODO: add ws after configuring zeromq connector
            let translator = ConnectorNNGPubSub::builder()
                .with_endpoint("tcp://0.0.0.0:5557".to_string())
                .with_handler(TranslatorCommand)
                .build_publisher()
                .bind()
                .into_inner();

            let agent_command = AgentCommand { translator };

            let db_service = ConnectorNNG::builder()
                .with_endpoint("tcp://0.0.0.0:5558".to_string())
                .with_handler(DummyTimescaleHandler)
                .with_proto(Proto::Pull)
                .build()
                .bind()
                .into_inner();
            
            let agent = ConnectorNNG::builder()
                .with_endpoint("tcp://0.0.0.0:5555".to_string())
                .with_handler(agent_command)
                .with_proto(Proto::Pull)
                .build()
                .bind()
                .into_inner();

            NngPoller::new()
                .add(agent)
                .add(db_service)
                .poll(-1);
        });
    }
}