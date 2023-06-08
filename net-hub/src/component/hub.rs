use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use log::{debug, info};
use threadpool::ThreadPool;
use net_core::{layer::NetComponent, transport::{sockets::Sender, dummy_command::DummyCommand}};

use net_core::transport::{
    connector_nng_pub_sub::ConnectorNNGPubSub,
    connector_nng::{ConnectorNNG, Proto}
};
use net_core::transport::polling::nng::NngPoller;

use crate::command::{agent::AgentCommand, dummy_timescale::DummyTimescaleHandler};
use crate::command::ws_server::WsServerCommand;
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
        let dummy = ConnectorNNG::builder()
            .with_handler(DummyCommand)
            .with_endpoint("inproc://dummy".to_string())
            .with_proto(Proto::Pull)
            .build()
            .bind()
            .into_inner();
        let ws_server_command = WsServerCommand::new(dummy)
            .bind(9091)
            .into_inner();
        let ws_server_command_clone = ws_server_command.clone();
        self.pool.execute(move || {
            ws_server_command_clone.poll(-1);
        });

        self.pool.execute(move || {
            // TODO: add ws after configuring zeromq connector
            let ws_server = ConnectorNNG::builder()
                .with_shared_handler(ws_server_command)
                .with_endpoint("tcp://0.0.0.0:5558".to_string())
                .with_proto(Proto::Pull)
                .build()
                .bind()
                .into_inner();
            let translator = ConnectorNNGPubSub::builder()
                .with_endpoint("tcp://0.0.0.0:5557".to_string())
                .with_handler(TranslatorCommand)
                .build_publisher()
                .bind()
                .into_inner();

            let agent_command = AgentCommand { translator };
            
            let agent = ConnectorNNG::builder()
                .with_endpoint("tcp://0.0.0.0:5555".to_string())
                .with_handler(agent_command)
                .with_proto(Proto::Pull)
                .build()
                .bind()
                .into_inner();

            NngPoller::new()
                .add(agent)
                .add(ws_server)
                .poll(-1);
        });
    }
}