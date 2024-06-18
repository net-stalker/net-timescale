-- Add up migration script here

-- Create the materialized view
CREATE MATERIALIZED VIEW IF NOT EXISTS Total_Http_Requests_Materialized_View
AS
SELECT
    date_trunc('minute', (Parsed_Data->'l1'->'frame'->>'frame.time')::TIMESTAMPTZ) AS Frametime,
    Tenant_ID,
    Network_ID,
    Parsed_Data->'l3'->'ip'->>'ip.src' AS Src_IP,
    Parsed_Data->'l3'->'ip'->>'ip.dst' AS Dst_IP,
    (Parsed_Data->'l1'->'frame'->>'frame.len')::INTEGER AS Packet_Length,
    Parsed_Data->'l5'->'http' AS Http_Part
FROM Traffic
WHERE
    Parsed_Data->'l3'->'ip'->>'ip.src' is not null
    AND Parsed_Data->'l3'->'ip'->>'ip.dst' is not null
    AND Parsed_Data->'l5'->'http' IS NOT NULL
    AND (Parsed_Data->'l5'->'http'->>'http.request')::BOOL
GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Http_Part
UNION
SELECT
    date_trunc('minute', (Parsed_Data->'l1'->'frame'->>'frame.time')::TIMESTAMPTZ) AS Frametime,
    Tenant_ID,
    Network_ID,
    Parsed_Data->'l3'->'ipv6'->>'ipv6.src' AS Src_IP,
    Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' AS Dst_IP,
    (Parsed_Data->'l1'->'frame'->>'frame.len')::INTEGER AS Packet_Length,
    Parsed_Data->'l5'->'http' AS Http_Part
FROM Traffic
WHERE
    Parsed_Data->'l3'->'ipv6'->>'ipv6.src' is not null
    AND Parsed_Data->'l3'->'ipv6'->>'ipv6.dst' is not null
    AND Parsed_Data->'l5'->'http' IS NOT NULL
    AND (Parsed_Data->'l5'->'http'->>'http.request')::BOOL
GROUP BY Frametime, Tenant_ID, Network_ID, Src_IP, Dst_IP, Packet_Length, Http_Part;

-- Create composite index on Tenant_ID and Network_ID
CREATE INDEX IF NOT EXISTS idx_total_http_requests_tenant_network ON Total_Http_Requests_Materialized_View USING HASH (Tenant_ID, Network_ID);

-- Create BRIN index on Frametime for space efficiency
CREATE INDEX IF NOT EXISTS idx_total_http_requests_frametime_brin ON Total_Http_Requests_Materialized_View USING BRIN (Frametime);

-- Create separate indexes on Src_IP and Dst_IP
CREATE INDEX IF NOT EXISTS idx_total_http_requests_src_dst_ip ON Total_Http_Requests_Materialized_View USING HASH (Src_IP, Dst_IP);

-- Create index on Packet_Length
CREATE INDEX IF NOT EXISTS idx_total_http_requests_packet_length ON Total_Http_Requests_Materialized_View (Packet_Length);

-- Create GIN index on Http_Part for JSONB operations
CREATE INDEX IF NOT EXISTS idx_total_http_requests_http_part_gin ON Total_Http_Requests_Materialized_View USING GIN (Http_Part);
