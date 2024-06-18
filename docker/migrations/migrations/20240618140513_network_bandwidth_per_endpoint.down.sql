-- Add down migration script here

-- Drop the indexes created on Network_Bandwidth_Per_Endpoint_Materialized_View
DROP INDEX IF EXISTS idx_network_bandwidth_per_endpoint_tenant_network;
DROP INDEX IF EXISTS idx_network_bandwidth_per_endpoint_frametime_brin;
DROP INDEX IF EXISTS idx_network_bandwidth_per_endpoint_src_dst_ip;
DROP INDEX IF EXISTS idx_network_bandwidth_per_endpoint_packet_length;
DROP INDEX IF EXISTS idx_network_bandwidth_per_endpoint_protocols;

-- Drop the materialized view Network_Bandwidth_Per_Endpoint_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Network_Bandwidth_Per_Endpoint_Materialized_View;
