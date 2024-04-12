use super::MaterializedView;

const CREATE_MATERIALIZED_VIEW_QUERY: &str = "
CREATE MATERIALIZED VIEW IF NOT EXISTS Network_Bandwidth_Per_Endpoint_Materialized_View
AS
SELECT
    Parsed_Data ->'l1'->'frame'->>'frame.time' AS Frametime,
    Tenant_ID,
    Network_ID,
    Parsed_Data->'l3'->'ip'->>'ip.src' as Src_IP,
    Parsed_Data->'l3'->'ip'->>'ip.dst' as Dst_IP,
    Parsed_Data->'l1'->'frame'->>'frame.len' as Packet_Length,
    Parsed_Data->'l1'->'frame'->>'frame.protocols' as Protocols
FROM Traffic
GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Protocols;
";

pub struct NetworkBandwidthPerEndpointMaterializedView {}

#[async_trait::async_trait]
impl MaterializedView for NetworkBandwidthPerEndpointMaterializedView {
    const CREATE_MATERIALIZED_VIEW_QUERY: &'static str = CREATE_MATERIALIZED_VIEW_QUERY;
}