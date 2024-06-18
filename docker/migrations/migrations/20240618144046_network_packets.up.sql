-- Add up migration script here

-- Create the materialized view
CREATE MATERIALIZED VIEW IF NOT EXISTS Network_Packets
AS
SELECT
    Traffic.Tenant_Id AS tenant_id,
    Traffic.Pcap_ID AS id,
    Traffic.Network_Id AS network_id,
    Traffic.Insertion_Time AS insertion_time,
    Traffic.Parsed_Data->'l3'->'ip'->>'ip.src' AS src,
    Traffic.Parsed_Data->'l3'->'ip'->>'ip.dst' AS dst,
    string_to_array(Traffic.Parsed_Data->'l1'->'frame'->>'frame.protocols', ':') AS protocols,
    Traffic.Parsed_Data AS json_data
FROM Traffic
WHERE
    Parsed_Data->'l3'->'ip'->>'ip.src' IS NOT NULL  
    AND Parsed_Data->'l3'->'ip'->>'ip.dst' IS NOT NULL
UNION
SELECT
    Traffic.Tenant_Id AS tenant_id,
    Traffic.Pcap_ID AS id,
    Traffic.Network_Id AS network_id,
    Traffic.Insertion_Time AS insertion_time,
    Traffic.Parsed_Data->'l3'->'ipv6'->>'ipv6.src' AS src,
    Traffic.Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' AS dst,
    string_to_array(Traffic.Parsed_Data->'l1'->'frame'->>'frame.protocols', ':') AS protocols,
    Traffic.Parsed_Data AS json_data
FROM Traffic
WHERE
    Parsed_Data->'l3'->'ipv6'->>'ipv6.src' IS NOT NULL  
    AND Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' IS NOT NULL;

-- Create composite index on Tenant_ID and Network_ID
CREATE INDEX IF NOT EXISTS idx_network_packets_tenant_network ON Network_Packets USING HASH (tenant_id, network_id);

-- Create index on Insertion_Time for efficient time-based queries
CREATE INDEX IF NOT EXISTS idx_network_packets_insertion_time ON Network_Packets USING BRIN (insertion_time);

-- Create separate indexes on src and dst
CREATE INDEX IF NOT EXISTS idx_network_packets_src_dst ON Network_Packets USING HASH (src, dst);

-- Create GIN index on protocols for array operations
CREATE INDEX IF NOT EXISTS idx_network_packets_protocols_gin ON Network_Packets USING GIN (protocols);

-- Create GIN index on json_data for JSONB operations
CREATE INDEX IF NOT EXISTS idx_network_packets_json_data_gin ON Network_Packets USING GIN (json_data);
