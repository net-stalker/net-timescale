-- Add down migration script here

-- Drop the indexes created on Http_Clients_Materialized_View
DROP INDEX IF EXISTS idx_http_clients_tenant_network;
DROP INDEX IF EXISTS idx_http_clients_frametime_brin;
DROP INDEX IF EXISTS idx_http_clients_src_dst_ip;
DROP INDEX IF EXISTS idx_http_clients_http_part;
DROP INDEX IF EXISTS idx_http_clients_packet_length;

-- Drop the materialized view Http_Clients_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Http_Clients_Materialized_View;
