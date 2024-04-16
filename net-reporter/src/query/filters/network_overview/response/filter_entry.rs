use net_reporter_api::api::network_overview_dashboard_filters::filter_entry::FilterEntryDTO;

#[derive(sqlx::FromRow, Clone, Debug)]
pub struct FilterEntryResponse {
    #[sqlx(rename = "Endpoint")]
    pub endpoint: String,
    #[sqlx(rename = "Protocols")]
    pub protocols: Vec<String>,
    #[sqlx(rename = "Total_Bytes")]
    pub total_bytes: i64,
}

impl From<FilterEntryResponse> for FilterEntryDTO {
    fn from(value: FilterEntryResponse) -> Self {
        FilterEntryDTO::new(&value.endpoint, &value.protocols, value.total_bytes)
    }
}
