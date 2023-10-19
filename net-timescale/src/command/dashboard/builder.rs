use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use net_transport::sockets::Sender;
use sqlx::Postgres;
use crate::command::executor::PoolWrapper;
use crate::persistence::ChartGenerator;

pub struct DashboardHandlerBuilder<T, CG>
    where
        T: Sender + ?Sized,
        CG: ChartGenerator + ?Sized,
{
    pool: Option<Arc<PoolWrapper<Postgres>>>,
    consumer: Option<Rc<T>>,
    chart_generators: HashMap<&'static str, Rc<CG>>,
}

impl<T, CG> Default for DashboardHandlerBuilder<T, CG>
where
    T: Sender + ?Sized,
    CG: ChartGenerator + ?Sized,
{
    fn default() -> Self {
        Self {
            pool: None,
            consumer: None,
            chart_generators: HashMap::default(),
        }
    }
}

impl<T, CG> DashboardHandlerBuilder<T, CG>
where
    T: Sender + ?Sized,
    CG: ChartGenerator + ?Sized,
{
    pub fn with_pool(mut self, pool: Arc<PoolWrapper<Postgres>>) -> Self {
        self.pool = Some(pool);
        self
    }
    pub fn with_consumer(mut self, consumer: Rc<T>) -> Self {
        self.consumer = Some(consumer);
        self
    }
    pub fn add_chart_generator(
        mut self,
        chart_generator: Rc<CG>
    ) -> Self
    {
        let _ = self.chart_generators.insert(chart_generator.get_requesting_type(), chart_generator);
        self
    }
    pub fn build(self) -> super::handler::DashboardHandler<T, CG> {
        super::handler::DashboardHandler::new(
            self.consumer.unwrap(),
            self.pool.unwrap(),
            self.chart_generators,
        )
    }
}