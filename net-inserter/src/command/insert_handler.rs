use async_std::task::block_on;

use std::sync::Arc;

use sqlx::Postgres;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::decoder_api::Decoder;

use net_transport::sockets::Handler;
use net_transport::sockets::Sender;
use net_transport::sockets::Receiver;

use crate::command::executor::PoolWrapper;
use crate::repository::insert;

pub struct InsertHandler {
    pool: Arc<PoolWrapper<Postgres>>,
}
impl InsertHandler {
    pub fn new(executor: Arc<PoolWrapper<Postgres>>) -> Self {
        InsertHandler { pool: executor }
    }
}
impl Handler for InsertHandler {
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data: Vec<u8> = receiver.recv();
        let envelope = Envelope::decode(data.as_slice());
        let pooled_connection = block_on(self.pool.get_connection());
        let mut transaction = match block_on(pooled_connection.begin()) {
            Ok(tra) => tra,
            Err(err) => {
                log::error!("Couldn't start transaction: {}", err);
                return;
            }
        };
        match block_on(insert::insert_network_packet_transaction(&mut transaction, envelope)) {
            Ok(rows_count) => {
                log::info!("{} rows were affected", rows_count.rows_affected());
                block_on(transaction.commit()).expect("transaction commit is expected");
            }
            Err(error) => {
                log::error!("{}", error);
            }
        };
    }
}