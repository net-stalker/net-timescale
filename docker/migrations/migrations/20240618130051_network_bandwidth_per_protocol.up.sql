-- Add up migration script here
-- Create the materialized view
CREATE MATERIALIZED VIEW IF NOT EXISTS Network_Bandwidth_Per_Protocol_Materialized_View
AS
SELECT
    (Parsed_Data->'l1'->'frame'->>'frame.time')::TIMESTAMPTZ AS Frametime,
    Tenant_ID,
    Network_ID,
    Parsed_Data->'l3'->'ip'->>'ip.src' AS Src_IP,
    Parsed_Data->'l3'->'ip'->>'ip.dst' AS Dst_IP,
    (Parsed_Data->'l1'->'frame'->>'frame.len')::INTEGER AS Packet_Length,
    Parsed_Data->'l1'->'frame'->>'frame.protocols' AS Protocols
FROM Traffic
WHERE
    Parsed_Data->'l3'->'ip'->>'ip.src' is not null
    AND Parsed_Data->'l3'->'ip'->>'ip.dst' is not null
GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Protocols
UNION
SELECT
    (Parsed_Data->'l1'->'frame'->>'frame.time')::TIMESTAMPTZ AS Frametime,
    Tenant_ID,
    Network_ID,
    Parsed_Data->'l3'->'ipv6'->>'ipv6.src' AS Src_IP,
    Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' AS Dst_IP,
    (Parsed_Data->'l1'->'frame'->>'frame.len')::INTEGER AS Packet_Length,
    Parsed_Data->'l1'->'frame'->>'frame.protocols' AS Protocols
FROM Traffic
WHERE
    Parsed_Data->'l3'->'ipv6'->>'ipv6.src' is not null
    AND Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' is not null
GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Protocols;

-- Create composite index on Tenant_ID and Network_ID
CREATE INDEX IF NOT EXISTS idx_network_bandwidth_per_protocol_tenant_network ON Network_Bandwidth_Per_Protocol_Materialized_View (Tenant_ID, Network_ID);

-- Create BRIN index on Frametime for space efficiency
CREATE INDEX IF NOT EXISTS idx_network_bandwidth_per_protocol_frametime_brin ON Network_Bandwidth_Per_Protocol_Materialized_View USING BRIN (Frametime);

-- Create separate indexes on Src_IP and Dst_IP
CREATE INDEX IF NOT EXISTS idx_network_bandwidth_per_protocol_src_dst_ip ON Network_Bandwidth_Per_Protocol_Materialized_View (Src_IP, Dst_IP);

-- Create index on Packet_Length
CREATE INDEX IF NOT EXISTS idx_network_bandwidth_per_protocol_packet_length ON Network_Bandwidth_Per_Protocol_Materialized_View (Packet_Length);

-- Create index on Protocols for string operations
CREATE INDEX IF NOT EXISTS idx_network_bandwidth_per_protocols ON Network_Bandwidth_Per_Protocol_Materialized_View USING GIN (STRING_TO_ARRAY(Protocols, ':'));
