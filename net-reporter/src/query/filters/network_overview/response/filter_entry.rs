use net_reporter_api::api::network_overview_dashboard_filters::filter_entry::FilterEntryDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct FilterEntryResponse {
    pub endpoint: String,
    pub protocols: Vec<String>,
    pub total_bytes: i64,
}

impl From<FilterEntryResponse> for FilterEntryDTO {
    fn from(value: FilterEntryResponse) -> Self {
        FilterEntryDTO::new(&value.endpoint, &value.protocols, value.total_bytes)
    }
}
