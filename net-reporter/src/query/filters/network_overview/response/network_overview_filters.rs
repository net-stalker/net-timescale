use net_reporter_api::api::network_overview_dashboard_filters::filter_entry::FilterEntryDTO;
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;

use super::filter_entry::FilterEntryResponse;


#[derive(Default, Clone, Debug)]
pub struct NetworkOverviewFiltersResponse {
    entries: Vec<FilterEntryResponse>
}

impl From<NetworkOverviewFiltersResponse> for NetworkOverviewDashboardFiltersDTO {
    fn from(value: NetworkOverviewFiltersResponse) -> Self {

        NetworkOverviewDashboardFiltersDTO::new(
            value.entries
                .into_iter()
                .map(|fitler_entry | fitler_entry.into())
                .collect::<Vec<FilterEntryDTO>>()
                .as_slice()
        )
    }
}

impl From<Vec<FilterEntryResponse>> for NetworkOverviewFiltersResponse {
    fn from(value: Vec<FilterEntryResponse>) -> Self {
        NetworkOverviewFiltersResponse {
            entries: value
        }
    }
}
