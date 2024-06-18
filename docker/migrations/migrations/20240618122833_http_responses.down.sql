-- Add down migration script here

-- Drop the indexes created on Http_Responses_Materialized_View
DROP INDEX IF EXISTS idx_http_responses_tenant_network;
DROP INDEX IF EXISTS idx_http_responses_frametime_brin;
DROP INDEX IF EXISTS idx_http_responses_src_dst_ip;
DROP INDEX IF EXISTS idx_http_responses_http_part;
DROP INDEX IF EXISTS idx_http_responses_packet_length;

-- Drop the materialized view Http_Responses_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Http_Responses_Materialized_View;
