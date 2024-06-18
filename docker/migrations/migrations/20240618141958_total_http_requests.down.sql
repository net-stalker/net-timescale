-- Add down migration script here
-- Drop the indexes created on Total_Http_Requests_Materialized_View
DROP INDEX IF EXISTS idx_total_http_requests_tenant_network;
DROP INDEX IF EXISTS idx_total_http_requests_frametime_brin;
DROP INDEX IF EXISTS idx_total_http_requests_src_dst_ip;
DROP INDEX IF EXISTS idx_total_http_requests_packet_length;
DROP INDEX IF EXISTS idx_total_http_requests_http_part_gin;

-- Drop the materialized view Total_Http_Requests_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Total_Http_Requests_Materialized_View;
