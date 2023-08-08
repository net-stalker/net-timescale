use std::sync::Arc;
use std::collections::HashMap;
use async_std::task;
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
    connection_pool: PoolWrapper<Postgres>,
    router: Arc<S>,
    sender: async_channel::Sender<Vec<u8>>,
    receiver: async_channel::Receiver<Vec<u8>>,
    tasks: HashMap<String, task::JoinHandle<()>>,
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
        Self {
            connection_pool,
            router,
            sender,
            receiver,
            tasks: HashMap::default(),
        }
    }
    pub fn builder() -> ListenHandlerBuilder<S> {
        ListenHandlerBuilder::<S>::new()
    }

    async fn dispatch_command(&mut self, command: &str, channel: &str) -> Result<(), sqlx::Error> {
        match command {
            "listen" => {
                log::info!("got listen, waiting for lock");
                if self.tasks.contains_key(channel) {
                    log::debug!("{} is already being listened", channel);
                    return Ok(());
                }
                let sender = self.sender.clone();
                let mut listener = PgListener::connect_with(
                    self.connection_pool.get_connection().await,
                ).await?;
                listener.listen(channel).await?;
                self.tasks.insert(
                    channel.to_owned(),
                    task::spawn(
                        async move {
                            ListenHandler::<S>::_poll(
                                -1,
                                sender,
                                listener,
                            ).await
                        }
                    )
                );
                Ok(())
            },
            "unlisten" => {
                if !self.tasks.contains_key(channel) {
                    log::debug!("{} is already stopped to be listened", channel);
                    return Ok(());
                }
                if let Some(channel_poller) = self.tasks.remove(channel) {
                    channel_poller
                        .cancel()
                        .await;
                }
                Ok(())
            },
            _ => {
                // TODO: it is weird to return Ok here because it is actual is an error
                // need to think about improving this behaviour
                log::error!("unknown api command {}", command);
                Ok(())
            }
        }
    }

    pub async fn poll(&mut self, poll_count: i64) {
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
                            if let Err(err) = self.dispatch_command(
                                envelope.get_type(),
                                String::from_utf8(envelope.get_data().to_vec()).unwrap().as_str())
                                .await {
                                log::error!("couldn't dispatch a command: {}", err);
                            }
                        }
                    }
                },
                Err(err) => {
                    log::error!("{}", err);
                }
            }
        }
    }

    async fn _poll(
        poll_count: i64,
        sender: async_channel::Sender<Vec<u8>>,
        mut listener: PgListener,
    ) {
        let mut count = 0;
        loop {
            if count == poll_count {
                break;
            }
            log::debug!("waiting for something in _poll");
            let notification = match listener.recv().await {
                Ok(notification) => {
                    log::debug!("got notification from {}", notification.channel());
                    notification
                },
                Err(err) => {
                    log::error!("error while receiving a notification {:?}", err);
                    continue;
                }
            };
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
