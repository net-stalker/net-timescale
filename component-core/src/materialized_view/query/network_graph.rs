use crate::materialized_view::core::common::MaterializedView;
use crate::materialized_view::core::common::MaterializedViewQueries;

const NAME: &str = "Network_Graph_Materialized_View";

#[derive(Default)]
pub struct NetworkGraphMaterializedView {}

impl MaterializedViewQueries for NetworkGraphMaterializedView {
    fn get_name(&self) -> String {
        NAME.to_owned()
    }

    fn get_creation_query(&self) -> String {
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
        ", self.get_name())
    }
}

#[async_trait::async_trait]
impl MaterializedView for NetworkGraphMaterializedView {}