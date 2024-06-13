use crate::materialized_view::core::common::MaterializedView;
use crate::materialized_view::core::common::MaterializedViewQueries;

const NAME: &str = "Total_Http_Requests_Materialized_View";

#[derive(Default)]
pub struct TotalHttpRequestsMaterializedView {}

impl MaterializedViewQueries for TotalHttpRequestsMaterializedView {
    fn get_name(&self) -> String {
        NAME.to_owned()
    }

    fn get_creation_query(&self) -> String {
        format!("
            CREATE MATERIALIZED VIEW IF NOT EXISTS {}
            AS
            SELECT
                date_trunc('minute', (Parsed_Data->'l1'->'frame'->>'frame.time')::TIMESTAMPTZ) AS Frametime,
                Tenant_ID,
                Network_ID,
                Parsed_Data->'l3'->'ip'->>'ip.src' AS Src_IP,
                Parsed_Data->'l3'->'ip'->>'ip.dst' AS Dst_IP,
                (Parsed_Data->'l1'->'frame'->>'frame.len')::INTEGER AS Packet_Length,
                Parsed_Data->'l5'->'http' AS Http_Part
            FROM Traffic
            WHERE
                Parsed_Data->'l3'->'ip'->>'ip.src' is not null
                AND Parsed_Data->'l3'->'ip'->>'ip.dst' is not null
                AND Parsed_Data->'l5'->'http' IS NOT NULL
                AND (Parsed_Data->'l5'->'http'->>'http.request')::BOOL
            GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Http_Part
            UNION
            SELECT
                date_trunc('minute', (Parsed_Data->'l1'->'frame'->>'frame.time')::TIMESTAMPTZ) AS Frametime,
                Tenant_ID,
                Network_ID,
                Parsed_Data->'l3'->'ipv6'->>'ipv6.src' AS Src_IP,
                Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' AS Dst_IP,
                (Parsed_Data->'l1'->'frame'->>'frame.len')::INTEGER AS Packet_Length,
                Parsed_Data->'l5'->'http' AS Http_Part
            FROM Traffic
            WHERE
                Parsed_Data->'l3'->'ipv6'->>'ipv6.src' is not null
                AND Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' is not null
                AND Parsed_Data->'l5'->'http' IS NOT NULL
                AND (Parsed_Data->'l5'->'http'->>'http.request')::BOOL
            GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Http_Part;
        ", self.get_name())
    }
}

#[async_trait::async_trait]
impl MaterializedView for TotalHttpRequestsMaterializedView {}