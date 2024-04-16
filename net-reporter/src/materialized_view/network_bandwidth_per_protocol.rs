use super::MaterializedView;

const CREATE_MATERIALIZED_VIEW_QUERY: &str = "
CREATE MATERIALIZED VIEW IF NOT EXISTS Network_Bandwidth_Per_Protocol_Materialized_View
AS
SELECT
    (Parsed_Data->'l1'->'frame'->>'frame.time')::TIMESTAMPTZ AS Frametime,
    Tenant_ID,
    Network_ID,
    Parsed_Data->'l3'->'ip'->>'ip.src' as Src_IP,
    Parsed_Data->'l3'->'ip'->>'ip.dst' as Dst_IP,
    (Parsed_Data->'l1'->'frame'->>'frame.len')::INTEGER as Packet_Length,
    Parsed_Data->'l1'->'frame'->>'frame.protocols' as Protocols
FROM Traffic
GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Protocols;
";

pub struct NetworkBandwidthPerProtocolMaterializedView {}

#[async_trait::async_trait]
impl MaterializedView for NetworkBandwidthPerProtocolMaterializedView {
    const CREATE_MATERIALIZED_VIEW_QUERY: &'static str = CREATE_MATERIALIZED_VIEW_QUERY;
}