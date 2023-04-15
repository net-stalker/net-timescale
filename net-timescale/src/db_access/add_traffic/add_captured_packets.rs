use std::sync::Arc;
use net_core::transport::sockets::Handler;
use postgres::types::ToSql;
use serde_json::Value;
use crate::command::executor::Executor;
use crate::command::dispatcher::PacketData;
use crate::db_access::{query_result, as_query, query};

pub struct AddCapturedPackets {
    pub executor: Executor
} 
pub struct UpdatedRows {
    pub rows: u64
}
impl query_result::QueryResultComponent for UpdatedRows {}
impl as_query::AsQuery for AddCapturedPackets {
    // for now we use QueryResult. TODO: make query services like a separeate components
    fn execute(&self, data: &[u8]) -> Result<query_result::QueryResult, &'static str> {
        let frame_data: PacketData = bincode::deserialize(&data).unwrap();
        let result = self.insert(frame_data);
        match result{
            Ok(rows_count) => { 
                log::info!("{} rows were updated", rows_count);
                query_result::QueryResult::builder().with_result(Arc::new(UpdatedRows {rows: rows_count})).build()
            }
            Err(error) => {
                log::error!("{}", error);
                query_result::QueryResult::builder().with_error("Couldn't add data into table").build()
            }
        }
    }
}  
struct AddPacketsQuery {
    pub raw_query: String,
    pub args: PacketData
}
impl AddPacketsQuery {
    pub fn new(args: PacketData) -> Self {
        AddPacketsQuery { 
            raw_query: "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)".to_owned(),
            args
        } 
    }
}
impl query::PostgresQuery for AddPacketsQuery {
    fn get_query(&self) -> (String, Vec<&(dyn ToSql + Sync)>) {
        let new_args: Vec<&(dyn ToSql + Sync)> = vec![
            &self.args.frame_time,
            &self.args.src_addr,
            &self.args.dst_addr,
            &self.args.json
        ];
        (
            self.raw_query.to_owned(),
            new_args
        )
    }
}
impl AddCapturedPackets {
    pub fn insert(&self, data: PacketData) -> Result<u64, postgres::Error> {
        let query = Box::new(AddPacketsQuery::new(data));
        self.executor.execute(query)
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}
impl Handler for AddCapturedPackets {
    fn handle(&self, receiver: &dyn net_core::transport::sockets::Receiver, sender: &dyn net_core::transport::sockets::Sender) {
        receiver.recv();
        log::info!("handle in AddCapturedPackets");
    }
}

#[cfg(test)]
mod tests{
    use crate::db_access::query::PostgresQuery;
