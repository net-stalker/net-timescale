use std::rc::Rc;
use async_std::task::block_on;

use chrono::Utc;
use chrono::TimeZone;
use chrono::DateTime;

use net_timescale_api::api::overview_dashboard_filters::filter_entry::FilterEntryDTO;
use net_timescale_api::api::overview_dashboard_filters::overview_dashboard_filters_request::OverviewDashboardFiltersRequestDTO;
use net_timescale_api::api::overview_dashboard_filters::overview_dashbord_filters::OverviewDashboardFiltersDTO;
use sqlx::Transaction;
use sqlx::Postgres;
use sqlx::Pool;

use net_proto_api::api::API;
use net_proto_api::decoder_api::Decoder;
use net_proto_api::envelope::envelope::Envelope;
use net_proto_api::typed_api::Typed;

use crate::persistence::ChartGenerator;
use crate::persistence::Persistence;
use crate::repository::overview_filters_entry::OverviewFiltersEntry;

#[derive(Default, Clone, Debug)]
pub struct PersistenceOverviewFilters {
    entries: Vec<OverviewFiltersEntry>
}

impl PersistenceOverviewFilters {
    pub fn into_wrapped(self) -> Rc<dyn ChartGenerator> {
        Rc::new(self)
    }
}

impl From<PersistenceOverviewFilters> for OverviewDashboardFiltersDTO {
    fn from(value: PersistenceOverviewFilters) -> Self {
        OverviewDashboardFiltersDTO::new(
            value.entries
                .into_iter()
                .map(| entry | entry.into())
                .collect::<Vec<FilterEntryDTO>>()
                .as_slice()
        )
    }
}

#[async_trait::async_trait]
impl Persistence for PersistenceOverviewFilters {
    async fn get_chart_dto(
        &self,
        connection: &Pool<Postgres>,
        data: &Envelope
    ) -> Result<Rc<dyn API>, String> {
        let group_id = data.get_group_id().ok();
        if data.get_type() != OverviewDashboardFiltersRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let filters_request = OverviewDashboardFiltersRequestDTO::decode(data.get_data());
        let start_date: DateTime<Utc> = Utc.timestamp_millis_opt(filters_request.get_start_date_time()).unwrap();
        let end_date: DateTime<Utc> = Utc.timestamp_millis_opt(filters_request.get_end_date_time()).unwrap();
        let filters: Vec<FilterEntryDTO> = match block_on(OverviewFiltersEntry::select_by_date_cut(
            connection,
            group_id,
            start_date,
            end_date,
        )) {
            Ok(filters) => filters.into_iter().map(|entry| entry.into()).collect(),
            Err(err) => return Err(format!("Couldn't query bandwidth_buckets: {err}"))
        };
        Ok(Rc::new(OverviewDashboardFiltersDTO::new(filters.as_slice())))
    }

    async fn transaction_get_chart_dto(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        data: &Envelope
    ) -> Result<Rc<dyn API>, String> {
        let group_id = data.get_group_id().ok();
        if data.get_type() != OverviewDashboardFiltersRequestDTO::get_data_type() {
            return Err(format!("wrong request is being received: {}", data.get_type()));
        }
        let filters_request = OverviewDashboardFiltersRequestDTO::decode(data.get_data());
        let start_date: DateTime<Utc> = Utc.timestamp_millis_opt(filters_request.get_start_date_time()).unwrap();
        let end_date: DateTime<Utc> = Utc.timestamp_millis_opt(filters_request.get_end_date_time()).unwrap();
        let filters: Vec<FilterEntryDTO> = match block_on(OverviewFiltersEntry::transaction_select_by_date_cut(
            transaction,
            group_id,
            start_date,
            end_date,
        )) {
            Ok(filters) => filters.into_iter().map(|entry| entry.into()).collect(),
            Err(err) => return Err(format!("Couldn't query bandwidth_buckets: {err}"))
        };
        Ok(Rc::new(OverviewDashboardFiltersDTO::new(filters.as_slice())))
    }
}
// TODO: having trait with method transaction_get_dto we can easily derive this method
impl ChartGenerator for PersistenceOverviewFilters {
    fn get_requesting_type(&self) -> &'static str where Self: Sized {
        // TODO: this method can also be derived somehow, probably by adding parameters into derive macro
        OverviewDashboardFiltersRequestDTO::get_data_type()
    }
}