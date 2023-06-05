use std::sync::Arc;

use chrono::{DateTime, Utc, TimeZone};
use net_core::transport::sockets::{Receiver, Sender, Handler};
use postgres::{types::ToSql, Row};
use crate::{command::executor::Executor, persistence::postgres_query};
use r2d2::ManageConnection;
use super::postgres_query::{TimeInterval, TimeIntervalQuery};

pub struct TimeIntervalHandler<T, M>
where
    T: Sender + ?Sized,
    M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    executor: Executor<M>,
    result_receiver: Arc<T>
}


impl<T, M> TimeIntervalHandler<T, M>
    where
        T: Sender + ?Sized,
        M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    pub fn new(executor: Executor<M>, result_receiver: Arc<T>) -> Self {
        TimeIntervalHandler {
            executor,
            result_receiver
        }
    }
    pub fn select_time_interval(&self, data: TimeInterval) -> Result<Vec<Row>, postgres::Error> {
        let start = Utc.timestamp_millis_opt(data.start_interval).unwrap();
        let end = Utc.timestamp_millis_opt(data.end_interval).unwrap();
        let query = TimeIntervalQuery::new(&start, &end);
        self.executor.query(&query)
    }
}
impl<T, M> Handler for TimeIntervalHandler<T, M>
    where
        T: Sender + ?Sized,
        M: ManageConnection<Connection = postgres::Client, Error = postgres::Error>
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        log::info!("received data in SelectInterval::handle: {:?}", data);
        todo!("wait for middleware format implementation");
    }
}
