-- Add down migration script here
-- Drop the indexes created on Http_Overview_Filters_Materialized_View
DROP INDEX IF EXISTS idx_http_overview_filters_tenant_network;
DROP INDEX IF EXISTS idx_http_overview_filters_frametime_brin;
DROP INDEX IF EXISTS idx_http_overview_filters_src_ip;
DROP INDEX IF EXISTS idx_http_overview_filters_dst_ip;
DROP INDEX IF EXISTS idx_http_overview_filters_packet_length;
DROP INDEX IF EXISTS idx_http_overview_filters_http_part_gin;

-- Drop the materialized view Http_Overview_Filters_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Http_Overview_Filters_Materialized_View;
