use std::sync::Arc;
use chrono::{DateTime, Local};
use postgres::{types::ToSql};
use serde_json::Value;
use super::{
    as_query::AsQuery,
    query::{self, PostgresParams}
};
use crate::command::executor::Executor;
use crate::command::dispatcher::FrameData;
use super::query_result::{QueryResult, QueryResultComponent};

pub struct AddCapturedPackets {
    // executor is thread safe by itself
    pub executor: Executor
}
// TODO: remove everything which is somehow related to QueryResult 
pub struct UpdatedRows {
    pub rows: u64
}
impl QueryResultComponent for UpdatedRows {}
impl AsQuery for AddCapturedPackets {
    fn execute(&self, data: &[u8]) -> Result<QueryResult, &'static str> {
        let frame_data: FrameData = bincode::deserialize(&data).unwrap();
        let result = self.insert(
            frame_data.frame_time.parse::<DateTime<Local>>().unwrap(),
            frame_data.src_addr,
            frame_data.dst_addr,
            frame_data.binary_json);
        match result{
            Ok(rows_count) => {
                // TODO: move logging into dispatcher 
                log::info!("{} rows were updated", rows_count);
                QueryResult::builder().with_result(Arc::new(UpdatedRows {rows: rows_count})).build()
            }
            Err(error) => {
                // TODO: move logging into dispatcher
                log::error!("{}", error);
                QueryResult::builder().with_error("Couldn't add data into table").build()
            }
        }
    }
}
// TODO: move this to a separate file
struct AddPacketsQuery<'a> {
    pub raw_query: &'a str,
    pub args: &'a[&'a(dyn postgres::types::ToSql + Sync)]
}
impl<'a> AddPacketsQuery<'a> {
    pub fn new(args: &'a[&'a(dyn postgres::types::ToSql + Sync)]) -> Self {
        AddPacketsQuery { 
            raw_query: "INSERT INTO CAPTURED_TRAFFIC (frame_time, src_addr, dst_addr, binary_data) VALUES ($1, $2, $3, $4)",
            args
        } 
    }
}
impl<'a> query::PostgresQuery<'a> for AddPacketsQuery<'a> {
    fn get_query(&self) -> (&'a str, &'a[&'a(dyn postgres::types::ToSql + Sync)]) {
        (self.raw_query, self.args)
    }
}
impl AddCapturedPackets {
    pub fn insert(&self, frame_time: DateTime<Local>, src_addr: String, dst_addr: String, packet_json: Vec<u8>) -> Result<u64, postgres::Error> {
        let json_value = Self::convert_to_value(packet_json).unwrap();
        // To avoid a lot of unnessesray info about ToSql + Sync consider creating some kind of trait wrapper
        // Something like `trait PostgresParams`
        let binding: [&(dyn postgres::types::ToSql + Sync); 4] = [&frame_time, &src_addr, &dst_addr, &json_value];
        let query = Box::new(AddPacketsQuery::new(&binding));
        self.executor.execute(query)
    }

    fn convert_to_value(packet_json: Vec<u8>) -> serde_json::Result<Value> {
        serde_json::from_slice(&*packet_json)
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_add_packet_query(){
        todo!()
    }
}