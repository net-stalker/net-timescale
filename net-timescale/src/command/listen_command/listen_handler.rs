use std::{
    sync::Arc,
};
use std::sync::atomic::{AtomicBool, Ordering};
use async_std::task;
use async_std::sync::RwLock;
use async_std::task::block_on;
use sqlx::{Postgres, postgres::PgListener};
use net_transport::sockets::Sender;
use net_proto_api::{
    decoder_api::Decoder,
    encoder_api::Encoder,
    envelope::envelope::Envelope,
};
use crate::internal_api::notification::NotificationDTO;
use crate::command::executor::PoolWrapper;

pub struct ListenHandler<S>
where S: Sender
{
    listener: Arc<RwLock<PgListener>>,
    router: Arc<S>,
}

// TODO: need to write integration tests
impl<S> ListenHandler<S>
    where S: Sender
{
    pub async fn new(
        connection_pool: PoolWrapper<Postgres>,
        router: Arc<S>,
    ) -> Self {
        let listener = PgListener::connect_with(connection_pool.get_connection().await)
            .await
            .expect("expected to construct listener");
        let listener = Arc::new(RwLock::new(listener));
        Self {
            listener,
            router,
        }
    }
    pub fn builder() -> ListenHandlerBuilder<S> {
        ListenHandlerBuilder::<S>::new()
    }

    async fn dispatch_command(&self, command: &str, channel: &str) {
        match command {
            "listen" => {
                let mut listener = self.listener.write().await;
                match listener.listen(channel).await {
                    Ok(_) => {
                        log::debug!("started listening on {}", channel);
                    },
                    Err(err) => {
                        log::error!("error while trying to listen on {}: {}", channel, err);
                    }
                }
            },
            "unlisten" => {
                let mut listener = self.listener.write().await;
                match listener.unlisten(channel).await {
                    Ok(_) => {
                        log::debug!("stopped listening on {}", channel);
                    },
                    Err(err) => {
                        log::error!("error while trying to stop listening on {}: {}", channel, err);
                    }
                }
            },
            _ => {
                log::error!("wrong api command {}", command);
            }
        }
    }

    pub async fn poll(&self, poll_count: i64) {
        let (sender, receiver) = async_channel::unbounded();
        let listener = self.listener.clone();
        let stopper = Arc::new(AtomicBool::new(false));
        let stopper_clone = stopper.clone();
        let poller = task::spawn(async move {
            ListenHandler::<S>::_poll(
                poll_count,
                sender,
                listener,
                stopper_clone
            ).await;
        });
        let mut count = 0;
        loop {
            if count == poll_count {
                break;
            }
            match receiver.recv().await {
                Ok(data) => {
                    let envelope = Envelope::decode(data.as_slice());
                    if envelope.get_type() == "notification" {
                        self.router.send(data.as_slice());
                    }
                    match envelope.get_type() {
                        "notification" => {
                            self.router.send(data.as_slice());
                            count += 1;
                        },
                        "close_all" => {
                            break;
                        }
                        _ => {
                            // do something here
                            self.dispatch_command("test", "test").await;
                        }
                    }
                },
                Err(err) => {
                    log::error!("{}", err);
                }
            }
        }
        stopper.store(true, Ordering::Relaxed);
        poller.await;
    }

    async fn _poll(
        poll_count: i64,
        sender: async_channel::Sender<Vec<u8>>,
        listener: Arc<RwLock<PgListener>>,
        stopper: Arc<AtomicBool>,
    ) {
        let mut count = 0;
        loop {
            if count == poll_count || stopper.load(Ordering::Relaxed) {
                break;
            }
            // use try_recv here
            let notification = match listener.write().await.recv().await {
                Ok(notification) => {
                    notification
                },
                Err(err) => {
                    log::error!("error while receiving a notification {:?}", err);
                    continue;
                }
            };

            // TODO: need to receive necessary info using payload
            let notification = NotificationDTO::new(notification.payload()).encode();
            let envelope = Envelope::new("notification", notification.as_slice()).encode();
            sender.send(envelope).await.unwrap();
            count += 1;
        }
    }
}


pub struct ListenHandlerBuilder<S>
where S: Sender
{
    pub connection_pool: Option<PoolWrapper<Postgres>>,
    pub router: Option<Arc<S>>,
}

impl<S> ListenHandlerBuilder<S>
where S: Sender {
    pub fn new() -> Self {
        Self {
            connection_pool: None,
            router: None,
        }
    }
    pub fn with_router(mut self, router: Arc<S>) -> Self {
        self.router = Some(router);
        self
    }
    pub fn with_connection_pool(mut self, connection_pool: PoolWrapper<Postgres>) -> Self {
        self.connection_pool = Some(connection_pool);
        self
    }
    pub fn build(mut self) -> ListenHandler<S> {
        block_on(ListenHandler::new(
            self.connection_pool.unwrap(),
            self.router.unwrap(),
        ))
    }
}
