use std::{
    collections::HashSet,
    sync::Arc,
};
use async_std::task;
use async_std::sync::RwLock;
use async_std::task::block_on;
use sqlx::{Postgres, postgres::PgListener};
use net_transport::sockets::Sender;
use net_proto_api::{
    encoder_api::Encoder,
    envelope::envelope::Envelope,
};
use crate::internal_api::notification::NotificationDTO;
use crate::command::executor::PoolWrapper;

pub struct ListenHandler<S>
where S: Sender
{
    pub connection_pool: PoolWrapper<Postgres>,
    pub connections: Arc<RwLock<HashSet<i64>>>,
    pub router: Arc<S>,
}
// TODO: need to write integration tests
impl<S> ListenHandler<S>
    where S: Sender
{
    pub fn new(
        connection_pool: PoolWrapper<Postgres>,
        connections: Arc<RwLock<HashSet<i64>>>,
        router: Arc<S>,
    ) -> Self {
        Self {
            connection_pool,
            connections,
            router,
        }
    }
    pub fn builder() -> ListenHandlerBuilder<S> {
        ListenHandlerBuilder::<S>::default()
    }

    pub async fn add_tenant(&mut self, connection: i64) {
        self.connections.write().await.insert(connection);
    }

    pub fn get_tenants(&mut self) -> Arc<RwLock<HashSet<i64>>> {
        self.connections.clone()
    }

    pub async fn remove_tenant(&mut self, connection: i64) -> bool {
        self.connections.write().await.remove(&connection)
    }

    pub async fn start( self, channel_to_listen: &str, poll_count: i64) {
        let mut listener = PgListener::connect_with(
            self.connection_pool.get_connection().await)
            .await.unwrap();
        listener.listen(channel_to_listen).await.unwrap();
        let (sender, receiver) = async_channel::unbounded();
        let tenants_clone = self.connections.clone();
        let poller = task::spawn(async move {
            ListenHandler::<S>::poll(
                poll_count,
                tenants_clone,
                sender,
                listener
            ).await;
        });
        let mut count = 0;
        loop {
            if count == poll_count {
                break;
            }
            match receiver.recv().await {
                Ok(data) => {
                    log::info!("connections in listen_handler {:?}", self.connections.read().await);
                    if !self.connections.read().await.is_empty() {
                        log::info!("notification in listen_handler");
                        self.router.send(data.as_slice());
                    } else {
                        log::error!("no connections");
                    }
                },
                Err(err) => {
                    log::error!("{}", err);
                }
            }
            count += poll_count;
        }
        poller.await;
    }

    async fn poll(
        poll_count: i64,
        _connections: Arc<RwLock<HashSet<i64>>>,
        sender: async_channel::Sender<Vec<u8>>,
        mut listener: PgListener,
    ) {
        let mut count = 0;
        loop {
            if count == poll_count {
                break;
            }
            let notification = match listener.recv().await {
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
            let envelope = Envelope::new(None, None, "notification", notification.as_slice()).encode();
            sender.send(envelope).await.unwrap();
            count += 1;
        }
    }
}


impl<S> Default for ListenHandlerBuilder<S>
where S: Sender
{
    fn default() -> Self {
        Self {
            connection_pool: None,
            connections: Arc::new(RwLock::new(HashSet::default())),
            router: None,
        }
    }
}

pub struct ListenHandlerBuilder<S>
where S: Sender
{
    pub connection_pool: Option<PoolWrapper<Postgres>>,
    pub connections: Arc<RwLock<HashSet<i64>>>,
    pub router: Option<Arc<S>>,
}

impl<S> ListenHandlerBuilder<S>
where S: Sender {

    pub fn with_router(mut self, router: Arc<S>) -> Self {
        self.router = Some(router);
        self
    }
    pub fn add_tenant(self, connection: i64) -> Self {
        block_on(self.connections.write()).insert(connection);
        self
    }
    pub fn with_connection_pool(mut self, connection_pool: PoolWrapper<Postgres>) -> Self {
        self.connection_pool = Some(connection_pool);
        self
    }
    pub fn build(self) -> ListenHandler<S> {
        ListenHandler::new(
            self.connection_pool.unwrap(),
            self.connections.clone(),
            self.router.unwrap(),
        )
    }
}
