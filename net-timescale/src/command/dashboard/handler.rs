use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Arc;
use async_std::task::block_on;
use net_transport::{
    sockets::{Handler, Receiver, Sender, Pub},
};
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::typed_api::Typed;
use net_proto_api::api::API;
use net_timescale_api::api::dashboard::DashboardDTO;
use net_timescale_api::api::dashboard_request::DashboardRequestDTO;
use sqlx::{Acquire, Database, Postgres};
use crate::command::dashboard::builder::DashboardHandlerBuilder;
use crate::command::executor::PoolWrapper;

pub struct DashboardHandler<T, C, DB>
where
    T: Sender + Pub + ?Sized,
    C: API + ?Sized,
    DB: Database
{
    consumer: Rc<T>,
    pool: Arc<PoolWrapper<DB>>,
    chart_constructors: HashMap<&'static str, fn(&mut sqlx::Transaction<DB>, &[u8]) -> Rc<C>>
}
impl<T, C, DB> DashboardHandler<T, C, DB>
where
    T: Sender + Pub + ?Sized,
    C: API + ?Sized,
    DB: Database,
{
    pub fn new(
        consumer: Rc<T>,
        pool: Arc<PoolWrapper<DB>>,
        chart_constructors: HashMap<&'static str, fn(&mut sqlx::Transaction<DB>, &[u8]) -> Rc<C>>
    ) -> Self {
        Self {
            consumer,
            pool,
            chart_constructors,
        }
    }
    pub fn builder() -> DashboardHandlerBuilder<T, C, DB> {
        DashboardHandlerBuilder::default()
    }
}
impl<T, C, DB> Handler for DashboardHandler<T, C, DB>
where
    T: Sender + Pub + ?Sized,
    C: API + ?Sized, DB: Database,
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(&data);
        let group_id = match envelope.get_group_id() {
            Ok(id) => Some(id),
            Err(_) => None
        };
        let agent_id = match envelope.get_agent_id() {
            Ok(id) => Some(id),
            Err(_) => None
        };
        if envelope.get_type() != DashboardRequestDTO::get_data_type() {
            log::error!("wrong data type is received in DashboardHandler: {}", envelope.get_type());
            return;
        }

        let dashboard_request = DashboardRequestDTO::decode(envelope.get_data());
        let chart_requests = dashboard_request.get_chart_requests();
        let mut transaction = block_on(block_on(self.pool.get_connection()).begin()).unwrap();
        let mut charts = Vec::with_capacity(chart_requests.len());
        chart_requests.iter().for_each(|request| {
            let chart = self.chart_constructors.get(request.get_type()).unwrap()(&mut transaction, request.get_data());
            charts.push(Envelope::new(
                group_id.clone(),
                agent_id.clone(),
                chart.get_type(),
                chart.encode().as_slice()));
        });
        let dashboard = DashboardDTO::new(charts.as_slice());
        let enveloped_dashboard = Envelope::new(
            group_id,
            agent_id,
            dashboard.get_type(),
            dashboard.encode().as_slice()
        );
        self.consumer.send(enveloped_dashboard.encode().as_slice());
    }
}