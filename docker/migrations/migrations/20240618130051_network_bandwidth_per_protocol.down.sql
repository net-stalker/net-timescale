-- Add down migration script here

-- Drop the indexes created on Network_Bandwidth_Per_Protocol_Materialized_View
DROP INDEX IF EXISTS idx_network_bandwidth_per_protocol_tenant_network;
DROP INDEX IF EXISTS idx_network_bandwidth_per_protocol_frametime_brin;
DROP INDEX IF EXISTS idx_network_bandwidth_per_protocol_src_dst_ip;
DROP INDEX IF EXISTS idx_network_bandwidth_per_protocol_packet_length;
DROP INDEX IF EXISTS idx_network_bandwidth_per_protocol_protocols;

-- Drop the materialized view Network_Bandwidth_Per_Protocol_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Network_Bandwidth_Per_Protocol_Materialized_View;
