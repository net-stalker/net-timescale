#![allow(clippy::type_complexity)]
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use net_proto_api::api::API;
use net_proto_api::envelope::envelope::Envelope;
use net_transport::sockets::Sender;
use sqlx::Database;
use crate::command::executor::PoolWrapper;

pub struct DashboardHandlerBuilder<T, C, DB>
where
    T: Sender + ?Sized,
    C: API + ?Sized,
    DB: Database
{
    pool: Option<Arc<PoolWrapper<DB>>>,
    consumer: Option<Rc<T>>,
    chart_constructor: Option<HashMap<&'static str, fn(&mut sqlx::Transaction<DB>, &Envelope) -> Result<Rc<C>, String>>>,
}

impl<T, C, DB> Default for DashboardHandlerBuilder<T, C, DB>
where
    T: Sender + ?Sized,
    C: API + ?Sized,
    DB: Database
{
    fn default() -> Self {
        Self {
            pool: None,
            consumer: None,
            chart_constructor: None,
        }
    }
}

impl<T, C, DB> DashboardHandlerBuilder<T, C, DB>
where
    T: Sender + ?Sized,
    C: API + ?Sized,
    DB: Database
{
    pub fn with_pool(mut self, pool: Arc<PoolWrapper<DB>>) -> Self {
        self.pool = Some(pool);
        self
    }
    pub fn with_consumer(mut self, consumer: Rc<T>) -> Self {
        self.consumer = Some(consumer);
        self
    }
    pub fn add_chart_constructor(
        mut self,
        chart_constructor: (&'static str, fn(&mut sqlx::Transaction<DB>, &Envelope) -> Result<Rc<C>, String>))
        -> Self
    {
        self.chart_constructor.as_mut().unwrap().insert(chart_constructor.0, chart_constructor.1).unwrap();
        self
    }
    pub fn build(self) -> super::handler::DashboardHandler<T, C, DB> {
        super::handler::DashboardHandler::new(
            self.consumer.unwrap(),
            self.pool.unwrap(),
            self.chart_constructor.unwrap(),
        )
    }
}