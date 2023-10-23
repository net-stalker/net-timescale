use std::rc::Rc;
use async_std::task::block_on;
use chrono::{DateTime, TimeZone, Utc};
use net_proto_api::api::API;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::typed_api::Typed;
use sqlx::{Pool, Postgres, Transaction};
use net_timescale_api::api::{
    bandwidth_per_endpoint::{
        endpoint::EndpointDTO,
        bandwidth_per_endpoint::BandwidthPerEndpointDTO,
    },
    bandwidth_per_endpoint_request::BandwidthPerEndpointRequestDTO,
};
use crate::repository::endpoint::Endpoint;

#[derive(Clone, Debug)]
pub struct PersistenceBandwidthPerEndpoint {
    endpoints: Vec<Endpoint>
}

impl From<PersistenceBandwidthPerEndpoint> for BandwidthPerEndpointDTO {
    fn from(value: PersistenceBandwidthPerEndpoint) -> Self {
        BandwidthPerEndpointDTO::new(
            value.endpoints
                .into_iter()
                .map(| endpoint | endpoint.into())
                .collect::<Vec<EndpointDTO>>()
                .as_slice()
        )
    }
}
impl PersistenceBandwidthPerEndpoint {
    pub async fn get_dto(connection: &Pool<Postgres>, data: &Envelope) -> Result<BandwidthPerEndpointDTO, String> {
        let group_id = data.get_group_id().ok();
        if data.get_type() != BandwidthPerEndpointRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let bw_request = BandwidthPerEndpointRequestDTO::decode(data.get_data());
        let start_date: DateTime<Utc> = Utc.timestamp_millis_opt(bw_request.get_start_date_time()).unwrap();
        let end_date: DateTime<Utc> = Utc.timestamp_millis_opt(bw_request.get_end_date_time()).unwrap();
        let endpoints: Vec<EndpointDTO> = match block_on(Endpoint::select_by_date_cut(
            connection,
            group_id,
            start_date,
            end_date,
        )) {
            Ok(endpoints) => endpoints.into_iter().map(|endpoint| endpoint.into()).collect(),
            Err(err) => return Err(format!("Couldn't query endpoints: {err}"))
        };
        Ok(BandwidthPerEndpointDTO::new(endpoints.as_slice()))
    }
    pub async fn transaction_get_dto(
        transaction: &mut Transaction<'_, Postgres>,
        data: &Envelope
    ) -> Result<BandwidthPerEndpointDTO, String> {
        let group_id = data.get_group_id().ok();
        if data.get_type() != BandwidthPerEndpointRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let bw_request = BandwidthPerEndpointRequestDTO::decode(data.get_data());
        let start_date: DateTime<Utc> = Utc.timestamp_millis_opt(bw_request.get_start_date_time()).unwrap();
        let end_date: DateTime<Utc> = Utc.timestamp_millis_opt(bw_request.get_end_date_time()).unwrap();
        let endpoints: Vec<EndpointDTO> = match block_on(Endpoint::transaction_select_by_date_cut(
            transaction,
            group_id,
            start_date,
            end_date,
        )) {
            Ok(endpoints) => endpoints.into_iter().map(|endpoint| endpoint.into()).collect(),
            Err(err) => return Err(format!("Couldn't query endpoints: {err}"))
        };
        Ok(BandwidthPerEndpointDTO::new(endpoints.as_slice()))
    }
}
// TODO: having trait with method transaction_get_dto we can easily derive this method
impl super::ChartGenerator for PersistenceBandwidthPerEndpoint {
    fn generate_chart(&self, transaction: &mut Transaction<Postgres>, data: &Envelope)
                      -> Result<Rc<dyn API>, String> where Self: Sized
    {
        match block_on(Self::transaction_get_dto(transaction, data)) {
            Ok(ng_dto) => Ok(Rc::new(ng_dto)),
            Err(err) => Err(err)
        }
    }
    fn get_requesting_type(&self) -> &'static str where Self: Sized {
        // TODO: this method can also be derived somehow, probably by adding parameters into derive macro
        BandwidthPerEndpointRequestDTO::get_data_type()
    }
}