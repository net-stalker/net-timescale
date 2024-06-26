-- Add down migration script here
-- Drop the indexes created on Network_Packets
DROP INDEX IF EXISTS idx_network_packets_tenant_network;
DROP INDEX IF EXISTS idx_network_packets_insertion_time;
DROP INDEX IF EXISTS idx_network_packets_src_dst;
DROP INDEX IF EXISTS idx_network_packets_protocols_gin;
DROP INDEX IF EXISTS idx_network_packets_json_data_gin;

-- Drop the materialized view Network_Packets
DROP MATERIALIZED VIEW IF EXISTS Network_Packets;
