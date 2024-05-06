use super::MaterializedView;
use super::MaterializedViewQueries;

const NAME: &str = "Http_Overview_Filters_Materialized_View";

pub struct HttpOverviewFiltersMaterializedView {}

impl MaterializedViewQueries for HttpOverviewFiltersMaterializedView {
    const NAME: &'static str = NAME;

    fn get_creation_query() -> String {
        format!("
            CREATE MATERIALIZED VIEW IF NOT EXISTS {}
            AS
            SELECT
                (Parsed_Data->'l1'->'frame'->>'frame.time')::TIMESTAMPTZ AS Frametime,
                Tenant_ID,
                Network_ID,
                Parsed_Data->'l3'->'ip'->>'ip.src' AS Src_IP,
                Parsed_Data->'l3'->'ip'->>'ip.dst' AS Dst_IP,
                (Parsed_Data->'l1'->'frame'->>'frame.len')::INTEGER AS Packet_Length,
                Parsed_Data->'l5'->'http' AS Http_Part
            FROM Traffic
            WHERE
                Parsed_Data->'l5'->'http' IS NOT NULL
            GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Http_Part;
        ", NAME)
    }
}

#[async_trait::async_trait]
impl MaterializedView for HttpOverviewFiltersMaterializedView {}