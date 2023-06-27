use std::sync::Arc;
use log::{info};
use threadpool::ThreadPool;
use net_core::{layer::NetComponent, transport::{sockets::Sender, dummy_command::DummyCommand}};

use net_core::transport::{
    connector_nng_pub_sub::ConnectorNNGPubSub,
    connector_nng::{ConnectorNNG, Proto},
};
use net_core::transport::zmq::builders::dealer::ConnectorZmqDealerBuilder;
use net_core::transport::polling::nng::NngPoller;
use net_core::transport::polling::zmq::ZmqPoller;

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
            .bind(self.config.frontend_gateway.ws_addr.clone())
            .into_inner();
        let ws_server_command_clone = ws_server_command.clone();
        self.pool.execute(move || {
            ws_server_command_clone.poll(-1);
        });
        let context = zmq::Context::new();
        self.pool.execute(move || {
            let timescale_router = ConnectorZmqDealerBuilder::new(context.clone())
                .with_handler(ws_server_command)
                .with_endpoint(self.config.timescale_router.addr)
                .build()
                .bind()
                .into_inner();

            let translator = ConnectorZmqDealerBuilder::new(context.clone())
                .with_endpoint(self.config.translator_gateway.addr)
                .with_handler(Arc::new(TranslatorCommand))
                .build()
                .bind()
                .into_inner();

            let agent_command = AgentCommand { translator };

            let agent = ConnectorZmqDealerBuilder::new(context.clone())
                .with_endpoint(self.config.agent_gateway.addr)
                .with_handler(Arc::new(agent_command))
                .build()
                .bind()
                .into_inner();

            ZmqPoller::new()
                .add(timescale_router)
                .add(agent)
                .poll(-1);
        });
    }
}