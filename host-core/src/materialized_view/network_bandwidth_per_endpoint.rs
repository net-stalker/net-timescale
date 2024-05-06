use super::MaterializedView;
use super::MaterializedViewQueries;

const NAME: &str = "Network_Bandwidth_Per_Endpoint_Materialized_View";

pub struct NetworkBandwidthPerEndpointMaterializedView {}

impl MaterializedViewQueries for NetworkBandwidthPerEndpointMaterializedView {
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
                Parsed_Data->'l1'->'frame'->>'frame.protocols' AS Protocols
            FROM Traffic
            GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Protocols;
        ", NAME)
    }
}

#[async_trait::async_trait]
impl MaterializedView for NetworkBandwidthPerEndpointMaterializedView {}