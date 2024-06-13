use crate::materialized_view::core::common::MaterializedView;
use crate::materialized_view::core::common::MaterializedViewQueries;

const NAME: &str = "Network_Packets";

#[derive(Default)]
pub struct NetworkPacketsMaterializedView {}

impl MaterializedViewQueries for NetworkPacketsMaterializedView {
    fn get_name(&self) -> String {
        NAME.to_owned()
    }

    fn get_creation_query(&self) -> String {
        format!("
            CREATE MATERIALIZED VIEW {}
            AS
            SELECT
                Traffic.Tenant_Id as tenant_id,
                Traffic.Pcap_ID AS id,
                Traffic.Network_Id AS network_id,
                Traffic.Insertion_Time AS insertion_time,
                Traffic.Parsed_Data->'l3'->'ip'->>'ip.src' AS src,
                Traffic.Parsed_Data->'l3'->'ip'->>'ip.dst' AS dst,
                string_to_array(Traffic.Parsed_Data->'l1'->'frame'->>'frame.protocols', ':') AS protocols,
                Traffic.Parsed_Data AS json_data
            FROM Traffic
            WHERE
                Parsed_Data->'l3'->'ip'->>'ip.src' is not null  
                AND Parsed_Data->'l3'->'ip'->>'ip.dst' is not null
            UNION
            SELECT
                Traffic.Tenant_Id as tenant_id,
                Traffic.Pcap_ID AS id,
                Traffic.Network_Id AS network_id,
                Traffic.Insertion_Time AS insertion_time,
                Traffic.Parsed_Data->'l3'->'ipv6'->>'ipv6.src' AS src,
                Traffic.Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' AS dst,
                string_to_array(Traffic.Parsed_Data->'l1'->'frame'->>'frame.protocols', ':') AS protocols,
                Traffic.Parsed_Data AS json_data
            FROM Traffic
            WHERE
                Parsed_Data->'l3'->'ipv6'->>'ipv6.src' is not null  
                AND Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' is not null;
        ", self.get_name())
    }
}

#[async_trait::async_trait]
impl MaterializedView for NetworkPacketsMaterializedView {}
