-- Add down migration script here

-- Drop the indexes created on Http_Responses_Distribution_Materialized_View
DROP INDEX IF EXISTS idx_http_responses_distribution_tenant_network;
DROP INDEX IF EXISTS idx_http_responses_distribution_frametime_brin;
DROP INDEX IF EXISTS idx_http_responses_distribution_src_dst_ip;
DROP INDEX IF EXISTS idx_http_responses_distribution_http_part;
DROP INDEX IF EXISTS idx_http_responses_distribution_packet_length;

-- Drop the materialized view Http_Responses_Distribution_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Http_Responses_Distribution_Materialized_View;
