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
use net_timescale_api::api::dashboard::DashboardDTO;
use net_timescale_api::api::dashboard_request::DashboardRequestDTO;
use sqlx::Postgres;
use crate::command::dashboard::builder::DashboardHandlerBuilder;
use crate::command::executor::PoolWrapper;
use crate::persistence::ChartGenerator;

pub struct DashboardHandler<T, CG>
where
    T: Sender + ?Sized,
    CG: ChartGenerator + ?Sized,
{
    consumer: Rc<T>,
    pool: Arc<PoolWrapper<Postgres>>,
    chart_generators: HashMap<&'static str, Rc<CG>>,
}
impl<T, CG> DashboardHandler<T, CG>
where
    T: Sender + ?Sized,
    CG: ChartGenerator + ?Sized,
{
    pub fn new(
        consumer: Rc<T>,
        pool: Arc<PoolWrapper<Postgres>>,
        chart_generators: HashMap<&'static str, Rc<CG>>,
    ) -> Self {
        Self {
            consumer,
            pool,
            chart_generators,
        }
    }
    pub fn builder() -> DashboardHandlerBuilder<T, CG> {
        DashboardHandlerBuilder::default()
    }
}
impl<T, CG> Handler for DashboardHandler<T, CG>
where
    T: Sender + ?Sized,
    CG: ChartGenerator + ?Sized,
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
            let generator = self.chart_generators.get(request.get_type()).unwrap();
            let chart = match generator.generate_chart(&mut transaction, request) {
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