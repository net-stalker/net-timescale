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
    listener: PgListener,
    router: Arc<S>,
    sender: async_channel::Sender<Vec<u8>>,
    receiver: async_channel::Receiver<Vec<u8>>,
}

// TODO: need to write integration tests
impl<S> ListenHandler<S>
    where S: Sender
{
    pub async fn new(
        connection_pool: PoolWrapper<Postgres>,
        sender: async_channel::Sender<Vec<u8>>,
        receiver: async_channel::Receiver<Vec<u8>>,
        router: Arc<S>,
    ) -> Self {
        let listener = PgListener::connect_with(connection_pool.get_connection().await)
            .await
            .expect("expected to construct listener");
        Self {
            listener,
            router,
            sender,
            receiver
        }
    }
    pub fn builder() -> ListenHandlerBuilder<S> {
        ListenHandlerBuilder::<S>::new()
    }

    async fn dispatch_command(&mut self, command: &str, channel: &str) {
        match command {
            "listen" => {
                log::info!("got listen, waiting for lock");
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
                // let mut listener = self.listener.write().await;
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

    pub async fn poll(&mut self, poll_count: i64) {
        let listener = self.listener;
        let stopper = Arc::new(AtomicBool::new(false));
        let stopper_clone = stopper.clone();
        let sender= self.sender.clone();
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
            match self.receiver.recv().await {
                Ok(data) => {
                    log::info!("got some data");
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
                        },
                        _ => {
                            log::info!("is dispatch command");
                            // do something here
                            self.dispatch_command(
                                envelope.get_type(),
                                String::from_utf8(envelope.get_data().to_vec()).unwrap().as_str())
                                .await;
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
            log::debug!("waiting for something in _poll");
            let notification = match listener.write().await.recv().await {
                Ok(notification) => {
                    log::debug!("got notification from {}", notification.channel());
                    notification
                },
                Err(err) => {
                    log::error!("error while receiving a notification {:?}", err);
                    continue;
                }
            };

            // TODO: need to receive necessary info using payload
            let notification = NotificationDTO::new(
                notification.payload(),
                notification.channel()).encode();
            let envelope = Envelope::new("notification", notification.as_slice()).encode();
            sender.send(envelope).await.unwrap();
            count += 1;
        }
    }
}


pub struct ListenHandlerBuilder<S>
where S: Sender
{
    connection_pool: Option<PoolWrapper<Postgres>>,
    router: Option<Arc<S>>,
    sender: Option<async_channel::Sender<Vec<u8>>>,
    receiver: Option<async_channel::Receiver<Vec<u8>>>,
}

impl<S> ListenHandlerBuilder<S>
where S: Sender {
    pub fn new() -> Self {
        Self {
            connection_pool: None,
            router: None,
            sender: None,
            receiver: None,
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
    pub fn with_sender(mut self, sender: async_channel::Sender<Vec<u8>>) -> Self {
        self.sender = Some(sender);
        self
    }
    pub fn with_receiver(mut self, receiver: async_channel::Receiver<Vec<u8>>) -> Self {
        self.receiver = Some(receiver);
        self
    }
    pub fn build(mut self) -> ListenHandler<S> {
        block_on(ListenHandler::new(
            self.connection_pool.unwrap(),
            self.sender.unwrap(),
            self.receiver.unwrap(),
            self.router.unwrap(),
        ))
    }
}
