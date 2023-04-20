use threadpool::ThreadPool;
use net_core::layer::NetComponent;

use net_core::transport::connector_nng::{ConnectorNNG, Proto};
use net_core::transport::polling::Poller;

use crate::command::decoder::DecoderCommand;
use crate::command::dummy::DummyCommand;

pub struct Translator {
    pub pool: ThreadPool,
}

impl Translator {
    pub fn new(pool: ThreadPool) -> Self {
        Self { pool }
    }
}

impl NetComponent for Translator {
    fn run(self) {
        log::info!("Run component");

        let push = ConnectorNNG::builder()
            .with_endpoint("tcp://0.0.0.0:5558".to_string())
            .with_proto(Proto::Req)
            .with_handler(DummyCommand)
            .build()
            .connect()
            .into_inner();
        let push_clone = push.clone();

        let server = ConnectorNNG::builder()
            .with_endpoint("tcp://0.0.0.0:5557".to_string())
            .with_proto(Proto::Rep)
            .with_handler(DecoderCommand { push: push_clone })
            .build()
            .bind()
            .into_inner();

        self.pool.execute(move || {
            Poller::new()
                .add(server)
                .add(push)
                .poll();
        });
    }
}
