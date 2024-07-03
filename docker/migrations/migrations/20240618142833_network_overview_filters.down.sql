-- Add down migration script here

-- Drop the indexes created on Network_Overview_Materialized_View
DROP INDEX IF EXISTS idx_network_overview_tenant_network;
DROP INDEX IF EXISTS idx_network_overview_frametime_brin;
DROP INDEX IF EXISTS idx_network_overview_src_ip;
DROP INDEX IF EXISTS idx_network_overview_dst_ip;
DROP INDEX IF EXISTS idx_network_overview_packet_length;
DROP INDEX IF EXISTS idx_network_overview_protocols;

-- Drop the materialized view Network_Overview_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Network_Overview_Filters_Materialized_View;