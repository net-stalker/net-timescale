-- Add down migration script here

-- Remove the scheduled cron job
SELECT cron.unschedule(cron.job_id)
FROM cron.job
WHERE jobname = 'delete_expired_records';

-- Drop the pg_cron extension
DROP EXTENSION IF EXISTS pg_cron;

-- Drop the trigger
DROP TRIGGER IF EXISTS set_delete_time_trigger ON Traffic_Buffer;

-- Drop the trigger function
DROP FUNCTION IF EXISTS set_delete_time();

-- Drop indexes for Traffic_Buffer
DROP INDEX IF EXISTS Pcap_Parsed_Data_Index;
DROP INDEX IF EXISTS Pcap_Tenant_ID_Index;
DROP INDEX IF EXISTS Pcap_Network_ID_Index;
DROP INDEX IF EXISTS Pcap_Insertion_Time_Index;
DROP INDEX IF EXISTS Pcap_ID_Index;

-- Drop the Traffic_Buffer table
DROP TABLE IF EXISTS Traffic_Buffer;

-- Drop indexes for Traffic
DROP INDEX IF EXISTS Pcap_Parsed_Data_Index;
DROP INDEX IF EXISTS Pcap_Tenant_ID_Index;
DROP INDEX IF EXISTS Pcap_Network_ID_Index;
DROP INDEX IF EXISTS Pcap_Insertion_Time_Index;
DROP INDEX IF EXISTS Pcap_ID_Index;

-- Drop the Traffic table
DROP TABLE IF EXISTS Traffic;

-- Drop indexes for Networks
DROP INDEX IF EXISTS Network_Tenant_ID_Index;
DROP INDEX IF EXISTS Network_Name_Index;
DROP INDEX IF EXISTS Network_ID_Index;

-- Drop the Networks table
DROP TABLE IF EXISTS Networks;
