use log::{info};
use threadpool::ThreadPool;
use net_core::{layer::NetComponent, transport::{sockets::Sender, dummy_command::DummyCommand}};

use net_core::transport::{
    connector_nng_pub_sub::ConnectorNNGPubSub,
    connector_nng::{ConnectorNNG, Proto},
};
use net_core::transport::connector_zeromq::ConnectorZmq;
use net_core::transport::polling::nng::NngPoller;

use crate::command::{agent::AgentCommand, dummy_timescale::DummyTimescaleHandler};
use crate::command::ws_server::WsServerCommand;
use crate::command::translator::TranslatorCommand;
use crate::config::Config;

pub struct Hub {
    pool: ThreadPool,
    config: Config,
}

impl Hub {
    pub fn new(pool: ThreadPool, config: Config) -> Self {
        Hub { pool, config }
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
                .with_endpoint(self.config.frontend_gateway.ws_addr)
                .with_proto(Proto::Pull)
                .build()
                .bind()
                .into_inner();

            let translator = ConnectorZmq::builder()
                .with_endpoint(self.config.translator_gateway.addr)
                .with_handler(TranslatorCommand)
                .build()
                .bind()
                .into_inner();

            let agent_command = AgentCommand { translator };

            let agent = ConnectorNNG::builder()
                .with_endpoint(self.config.agent_gateway.addr)
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