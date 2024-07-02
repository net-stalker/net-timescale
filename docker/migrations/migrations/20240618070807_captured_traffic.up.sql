-- Add up migration script here
CREATE TABLE IF NOT EXISTS Networks
(
    Network_ID          TEXT,
    Network_Name        TEXT NOT NULL,
    Tenant_ID           TEXT NOT NULL,
    Network_Color       TEXT NOT NULL,

    PRIMARY KEY (Network_ID),
    
    UNIQUE (Network_Name, Tenant_ID)
);

CREATE INDEX IF NOT EXISTS Network_ID_Index ON Networks USING HASH (Network_ID);
CREATE INDEX IF NOT EXISTS Network_Name_Index ON Networks USING HASH (Network_Name);
CREATE INDEX IF NOT EXISTS Network_Tenant_ID_Index ON Networks USING HASH (Tenant_ID);

CREATE TABLE IF NOT EXISTS Traffic
(
    Pcap_ID             TEXT,
    Insertion_Time      TIMESTAMPTZ NOT NULL,
    Network_ID          TEXT,
    Tenant_ID           TEXT NOT NULL,
    Raw_Pcap_File_Path  TEXT NOT NULL,
    Parsed_Data         JSONB NOT NULL,
    Delete_At           TIMESTAMPTZ NOT NULL,

    PRIMARY KEY (Pcap_ID),

    FOREIGN KEY (Network_ID) REFERENCES Networks(Network_ID) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS Pcap_ID_Index ON Traffic USING HASH (Pcap_ID);
CREATE INDEX IF NOT EXISTS Pcap_Insertion_Time_Index ON Traffic USING BRIN (Insertion_Time);
CREATE INDEX IF NOT EXISTS Pcap_Network_ID_Index ON Traffic USING HASH (Network_ID);
CREATE INDEX IF NOT EXISTS Pcap_Tenant_ID_Index ON Traffic USING HASH (Tenant_ID);
CREATE INDEX IF NOT EXISTS Pcap_Parsed_Data_Index ON Traffic USING GIN (Parsed_Data);

CREATE TABLE IF NOT EXISTS Traffic_Buffer
(
    Pcap_ID             TEXT,
    Insertion_Time      TIMESTAMPTZ NOT NULL,
    Network_ID          TEXT,
    Tenant_ID           TEXT NOT NULL,
    Raw_Pcap_File_Path  TEXT NOT NULL,
    Parsed_Data         JSONB NOT NULL,
    Delete_At           TIMESTAMPTZ NOT NULL,

    PRIMARY KEY (Pcap_ID),

    FOREIGN KEY (Network_ID) REFERENCES Networks(Network_ID) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS Pcap_ID_Index ON Traffic_Buffer USING HASH (Pcap_ID);
CREATE INDEX IF NOT EXISTS Pcap_Insertion_Time_Index ON Traffic_Buffer USING BRIN (Insertion_Time);
CREATE INDEX IF NOT EXISTS Pcap_Network_ID_Index ON Traffic_Buffer USING HASH (Network_ID);
CREATE INDEX IF NOT EXISTS Pcap_Tenant_ID_Index ON Traffic_Buffer USING HASH (Tenant_ID);
CREATE INDEX IF NOT EXISTS Pcap_Parsed_Data_Index ON Traffic_Buffer USING GIN (Parsed_Data);

-- Enable the pg_cron extension
CREATE EXTENSION IF NOT EXISTS pg_cron;

CREATE OR REPLACE FUNCTION delete_expired_records_and_refresh_views() RETURNS void AS $$
DECLARE
  matview RECORD;
  deleted_count INT;
BEGIN
   -- Perform the actual delete operation
  DELETE FROM Traffic WHERE Delete_At < NOW();
  DELETE FROM Traffic_Buffer WHERE Delete_At < NOW();
  
  -- Loop through each materialized view and refresh it
  FOR matview IN SELECT matviewname FROM pg_matviews LOOP
    EXECUTE 'REFRESH MATERIALIZED VIEW ' || quote_ident(matview.matviewname);
  END LOOP;
END;
$$ LANGUAGE plpgsql;

-- Schedule the cron job
SELECT cron.schedule('delete_expired_records', '0 0 * * *', $$SELECT delete_expired_records_and_refresh_views();$$);


CREATE OR REPLACE FUNCTION set_delete_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.Delete_At := NOW() + INTERVAL '1 day';
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_delete_time_trigger
BEFORE INSERT OR UPDATE ON Traffic_Buffer
FOR EACH ROW
EXECUTE FUNCTION set_delete_time();

