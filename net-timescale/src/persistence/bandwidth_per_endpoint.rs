use async_std::task::block_on;
use chrono::{TimeZone, Utc};
use sqlx::{Error, Postgres, Transaction};
use net_timescale_api::api::{
    bandwidth_per_endpoint::{
        endpoint::EndpointDTO,
        bandwidth_per_endpoint::BandwidthPerEndpointDTO,
    },
    bandwidth_per_endpoint_request::BandwidthPerEndpointDTO,
};
use crate::repository::endpoint;
use crate::repository::endpoint::Endpoint;

#[derive(Clone, Debug)]
pub struct TotalBytes {
    endpoints: Vec<Endpoint>
}

impl From<TotalBytes> for TotalBytesDTO {
    fn from(value: TotalBytes) -> Self {
        TotalBytesDTO::new(
            value.endpoints
                .into_iter()
                .map(| endpoint | endpoint.into())
                .collect::<Vec<EndpointDTO>>()
                .as_slice()
        )
    }
}
impl TotalBytes {
    pub fn select_total_bytes(
        transaction: &mut Transaction<'_, Postgres>,
        request: &TotalBytesRequestDTO
    ) -> Result<Self, Error> {
        let endpoints = match block_on(endpoint::select_by_date_cut(
            transaction,
            Utc.timestamp_nanos(request.get_start_date_time()),
            Utc.timestamp_nanos(request.get_end_date_time()),
        )) {
            Ok(endpoints) => endpoints,
            Err(err) => return Err(err)
        };
        Ok(Self { endpoints })
    }
}
