-- Add down migration script here
-- Drop the indexes created on Network_Bandwidth_Materialized_View
DROP INDEX IF EXISTS idx_network_bandwidth_tenant_network;
DROP INDEX IF EXISTS idx_network_bandwidth_frametime_brin;
DROP INDEX IF EXISTS idx_network_bandwidth_src_dst_ip;
DROP INDEX IF EXISTS idx_network_bandwidth_packet_length;
DROP INDEX IF EXISTS idx_network_bandwidth_protocols_gin;

-- Drop the materialized view Network_Bandwidth_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Network_Bandwidth_Materialized_View;
