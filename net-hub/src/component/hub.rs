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

use crate::command::{agent::AgentCommand, timescale_router_handler::TimescaleRouter};
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
pub const WS_CONSUMER: &'static str = "inproc://ws/consumer";
pub const WS_PRODUCER: &'static str = "inproc://ws/producer";

impl NetComponent for Hub {
    fn run(self) {
        info!("run component");
        let ws_consumer = ConnectorNNG::builder()
            .with_endpoint(WS_PRODUCER.to_string())
            .with_handler(DummyCommand)
            .with_proto(Proto::Push)
            .build()
            .bind()
            .into_inner();
        let ws_server_command = WsServerCommand::new(ws_consumer)
            .bind(self.config.frontend_gateway.ws_addr)
            .into_inner();
        let ws_server_command_clone = ws_server_command.clone();
        self.pool.execute(move || {
            ws_server_command_clone.poll(-1);
        });
        let context = zmq::Context::new();
        let context_clone = context.clone();
        self.pool.execute(move || {
            let ws_producer = ConnectorNNG::builder()
                .with_endpoint(WS_CONSUMER.to_string())
                .with_shared_handler(ws_server_command)
                .with_proto(Proto::Pull)
                .build()
                .bind()
                .into_inner();

            let timescale = ConnectorZmqDealerBuilder::new(context_clone)
                .with_endpoint("tcp://0.0.0.0:5557".to_string())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();

            let timescale_producer = ConnectorNNG::builder()
                .with_endpoint(WS_PRODUCER.to_string())
                .with_handler(TimescaleRouter {consumer: timescale})
                .with_proto(Proto::Pull)
                .build()
                .connect()
                .into_inner();

            NngPoller::new()
                .add(timescale_producer)
                .add(ws_producer)
                .poll(-1);
        });
        let context_clone = context.clone();
        self.pool.execute(move || {
            let ws_consumer = ConnectorNNG::builder()
                .with_endpoint(WS_CONSUMER.to_string())
                .with_handler(DummyCommand)
                .with_proto(Proto::Push)
                .build()
                .connect()
                .into_inner();
            let timescale_router = ConnectorZmqDealerBuilder::new(context_clone.clone())
                .with_handler(Arc::new(TimescaleRouter {consumer: ws_consumer}))
                .with_endpoint(self.config.timescale_router.addr)
                .build()
                .bind()
                .into_inner();

            ZmqPoller::new()
                .add(timescale_router)
                .poll(-1);
        });
        self.pool.execute(move || {
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
                .add(agent)
                .poll(-1);
        });
    }
}