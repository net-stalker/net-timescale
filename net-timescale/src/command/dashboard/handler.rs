#![allow(clippy::type_complexity)]
use std::collections::HashMap;

use std::rc::Rc;
use std::sync::Arc;
use async_std::task::block_on;
use net_transport::{
    sockets::{Handler, Receiver, Sender},
};
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::encoder_api::Encoder;
use net_proto_api::typed_api::Typed;
use net_proto_api::api::API;
use net_timescale_api::api::dashboard::DashboardDTO;
use net_timescale_api::api::dashboard_request::DashboardRequestDTO;
use sqlx::Database;
use crate::command::dashboard::builder::DashboardHandlerBuilder;
use crate::command::executor::PoolWrapper;

pub struct DashboardHandler<T, C, DB>
where
    T: Sender + ?Sized,
    C: API + ?Sized,
    DB: Database
{
    consumer: Rc<T>,
    pool: Arc<PoolWrapper<DB>>,
    chart_constructors: HashMap<&'static str, fn(&mut sqlx::Transaction<DB>, &Envelope) -> Result<Rc<C>, String>>,
}
impl<T, C, DB> DashboardHandler<T, C, DB>
where
    T: Sender + ?Sized,
    C: API + ?Sized,
    DB: Database,
{
    pub fn new(
        consumer: Rc<T>,
        pool: Arc<PoolWrapper<DB>>,
        chart_constructors: HashMap<&'static str, fn(&mut sqlx::Transaction<DB>, &Envelope) -> Result<Rc<C>, String>>,
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
    T: Sender + ?Sized,
    C: API + ?Sized, DB: Database,
{
    fn handle(&self, receiver: &dyn Receiver, _sender: &dyn Sender) {
        let data = receiver.recv();
        let envelope = Envelope::decode(&data);
        if envelope.get_type() != DashboardRequestDTO::get_data_type() {
            log::error!("wrong data type is received in DashboardHandler: {}", envelope.get_type());
            return;
        }

        let dashboard_request = DashboardRequestDTO::decode(envelope.get_data());
        log::debug!("dashboard request {:?}", dashboard_request);
        let chart_requests = dashboard_request.get_chart_requests();
        let mut transaction = block_on(block_on(self.pool.get_connection()).begin()).unwrap();
        let mut charts = Vec::with_capacity(chart_requests.len());
        for request in chart_requests.iter() {
            let chart = match self.chart_constructors.get(request.get_type()).unwrap()(&mut transaction, request) {
                Ok(chart) => chart,
                Err(err) => {
                    log::error!("{err}");
                    let _ = block_on(transaction.rollback());
                    return;
                }
            };
            log::debug!("got chart {}", chart.get_type());
            charts.push(Envelope::new(
                request.get_group_id().ok(),
                request.get_agent_id().ok(),
                chart.get_type(),
                chart.encode().as_slice()));
        }
        let _ = block_on(transaction.commit());
        let dashboard = DashboardDTO::new(charts.as_slice());
        let enveloped_dashboard = Envelope::new(
            envelope.get_group_id().ok(),
            envelope.get_agent_id().ok(),
            dashboard.get_type(),
            dashboard.encode().as_slice()
        );
        self.consumer.send(enveloped_dashboard.encode().as_slice());
    }
}