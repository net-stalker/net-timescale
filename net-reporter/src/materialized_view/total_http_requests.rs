use super::MaterializedView;

const CREATE_MATERIALIZED_VIEW_QUERY: &str = "
CREATE MATERIALIZED VIEW IF NOT EXISTS Total_Http_Requests_Materialized_View
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
    AND (Parsed_Data->'l5'->'http'->>'http.request')::BOOL
GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Http_Part;
";

pub struct TotalHttpRequestsMaterializedView {}

#[async_trait::async_trait]
impl MaterializedView for TotalHttpRequestsMaterializedView {
    const CREATE_MATERIALIZED_VIEW_QUERY: &'static str = CREATE_MATERIALIZED_VIEW_QUERY;
}