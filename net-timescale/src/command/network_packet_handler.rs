use std::sync::Arc;
use async_std::task::block_on;
use net_transport::sockets::{Handler, Receiver, Sender};
use net_proto_api::decoder_api::Decoder;
use sqlx::Postgres;
use net_proto_api::envelope::envelope::Envelope;
use crate::{command::executor::PoolWrapper, repository::network_packet};

pub struct NetworkPacketHandler {
    pool: Arc<PoolWrapper<Postgres>>,
}
impl NetworkPacketHandler {
    pub fn new(executor: Arc<PoolWrapper<Postgres>>) -> Self {
        NetworkPacketHandler { pool: executor }
    }
}
impl Handler for NetworkPacketHandler {
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
        match block_on(network_packet::insert_network_packet_transaction(&mut transaction, envelope)) {
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