use std::sync::Arc;
use std::time::Duration;
use log::{info};
use threadpool::ThreadPool;
use net_core::{layer::NetComponent, transport::{sockets::Sender, dummy_command::DummyCommand}};

use net_core::transport::zmq::builders::dealer::ConnectorZmqDealerBuilder;
use net_core::transport::polling::zmq::ZmqPoller;
use net_core::transport::zmq::contexts::dealer::DealerContext;

use crate::command::{agent::AgentCommand, router::Router};
use crate::command::ws_server::WsServerCommand;
use crate::command::translator::TranslatorCommand;
use crate::command::ws_router::WsRouter;
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
pub const WS_SERVER: &'static str = "ws://0.0.0.0:9091";

impl NetComponent for Hub {
    fn run(self) {
        info!("run component");
        let context = DealerContext::default();
        let context_clone = context.clone();

        self.pool.execute(move || {
            let ws_consumer = ConnectorZmqDealerBuilder::new(&context_clone)
                .with_endpoint("tcp://0.0.0.0:5557".to_string())
                .with_handler(Arc::new(DummyCommand))
                .build()
                .connect()
                .into_inner();
            let ws_server_command = WsServerCommand::new(ws_consumer)
                .bind(self.config.frontend_gateway.ws_addr)
                .into_inner();
            ws_server_command.poll(-1);
        });
        let context_clone = context.clone();

        self.pool.execute(move || {
            std::thread::sleep(Duration::from_secs(1));
            let ws_router = WsRouter::new(WS_SERVER);
            let timescale_router = ConnectorZmqDealerBuilder::new(&context_clone)
                .with_handler(Arc::new(ws_router))
                .with_endpoint(self.config.timescale_router.addr)
                .build()
                .bind()
                .into_inner();
            ZmqPoller::new()
                .add(timescale_router)
                .poll(-1);
        });

        self.pool.execute(move || {
            let translator = ConnectorZmqDealerBuilder::new(&context)
                .with_endpoint(self.config.translator_gateway.addr)
                .with_handler(Arc::new(TranslatorCommand))
                .build()
                .bind()
                .into_inner();

            let agent_command = AgentCommand { translator };

            let agent = ConnectorZmqDealerBuilder::new(&context)
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