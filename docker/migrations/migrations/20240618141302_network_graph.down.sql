-- Add down migration script here

-- Drop the indexes created on Network_Graph_Materialized_View
DROP INDEX IF EXISTS idx_network_graph_tenant_network;
DROP INDEX IF EXISTS idx_network_graph_frametime_brin;
DROP INDEX IF EXISTS idx_network_graph_src_dst_ip;
DROP INDEX IF EXISTS idx_network_graph_packet_length;
DROP INDEX IF EXISTS idx_network_graph_protocols;

-- Drop the materialized view Network_Graph_Materialized_View
DROP MATERIALIZED VIEW IF EXISTS Network_Graph_Materialized_View;
