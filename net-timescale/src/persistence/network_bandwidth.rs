use std::rc::Rc;
use async_std::task::block_on;

use chrono::Utc;
use chrono::TimeZone;
use chrono::DateTime;

use net_timescale_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_timescale_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;
use sqlx::Transaction;
use sqlx::Postgres;
use sqlx::Pool;

use net_proto_api::api::API;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::typed_api::Typed;

use net_timescale_api::api::network_bandwidth::bandwidth_bucket::BandwidthBucketDTO;

use crate::persistence::ChartGenerator;
use crate::persistence::Persistence;
use crate::repository::bandwidth_bucket::BandwidthBucket;

#[derive(Default, Clone, Debug)]
pub struct PersistenceNetworkBandwidth {
    bandwidth_buckets: Vec<BandwidthBucket>
}

impl PersistenceNetworkBandwidth {
    pub fn into_wrapped(self) -> Rc<dyn ChartGenerator> {
        Rc::new(self)
    }
}

impl From<PersistenceNetworkBandwidth> for NetworkBandwidthDTO {
    fn from(value: PersistenceNetworkBandwidth) -> Self {
        NetworkBandwidthDTO::new(
            value.bandwidth_buckets
                .into_iter()
                .map(| bandwidth_bucket | bandwidth_bucket.into())
                .collect::<Vec<BandwidthBucketDTO>>()
                .as_slice()
        )
    }
}

#[async_trait::async_trait]
impl Persistence for PersistenceNetworkBandwidth {
    async fn get_chart_dto(
        &self,
        connection: &Pool<Postgres>,
        data: &Envelope
    ) -> Result<Rc<dyn API>, String> {
        let group_id = data.get_group_id().ok();
        if data.get_type() != NetworkBandwidthRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let bw_request = NetworkBandwidthRequestDTO::decode(data.get_data());
        let start_date: DateTime<Utc> = Utc.timestamp_millis_opt(bw_request.get_start_date_time()).unwrap();
        let end_date: DateTime<Utc> = Utc.timestamp_millis_opt(bw_request.get_end_date_time()).unwrap();
        let bandwidth_buckets: Vec<BandwidthBucketDTO> = match block_on(BandwidthBucket::select_by_date_cut(
            connection,
            group_id,
            start_date,
            end_date,
        )) {
            Ok(bandwidth_buckets) => bandwidth_buckets.into_iter().map(|bandwidth_bucket| bandwidth_bucket.into()).collect(),
            Err(err) => return Err(format!("Couldn't query bandwidth_buckets: {err}"))
        };
        Ok(Rc::new(NetworkBandwidthDTO::new(bandwidth_buckets.as_slice())))
    }

    async fn transaction_get_chart_dto(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        data: &Envelope
    ) -> Result<Rc<dyn API>, String> {
        let group_id = data.get_group_id().ok();
        if data.get_type() != NetworkBandwidthRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let bw_request = NetworkBandwidthRequestDTO::decode(data.get_data());
        let start_date: DateTime<Utc> = Utc.timestamp_millis_opt(bw_request.get_start_date_time()).unwrap();
        let end_date: DateTime<Utc> = Utc.timestamp_millis_opt(bw_request.get_end_date_time()).unwrap();
        let bandwidth_buckets: Vec<BandwidthBucketDTO> = match block_on(BandwidthBucket::transaction_select_by_date_cut(
            transaction,
            group_id,
            start_date,
            end_date,
        )) {
            Ok(bandwidth_buckets) => bandwidth_buckets.into_iter().map(|bandwidth_bucket| bandwidth_bucket.into()).collect(),
            Err(err) => return Err(format!("Couldn't query bandwidth_buckets: {err}"))
        };
        Ok(Rc::new(NetworkBandwidthDTO::new(bandwidth_buckets.as_slice())))
    }
}
// TODO: having trait with method transaction_get_dto we can easily derive this method
impl super::ChartGenerator for PersistenceNetworkBandwidth {
    fn get_requesting_type(&self) -> &'static str where Self: Sized {
        // TODO: this method can also be derived somehow, probably by adding parameters into derive macro
        NetworkBandwidthRequestDTO::get_data_type()
    }
}