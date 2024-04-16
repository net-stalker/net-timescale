CREATE TABLE Networks
(
    Network_ID          SERIAL,
    Network_Name        TEXT NOT NULL,
    Tenant_ID           TEXT NOT NULL,
    Network_Color       TEXT NOT NULL,

    PRIMARY KEY (Network_ID),
    
    UNIQUE (Network_Name, Tenant_ID)
);

CREATE TABLE Traffic
(
    Pcap_ID             SERIAL,
    Insertion_Time      TIMESTAMPTZ NOT NULL,
    Network_ID          INT,
    Tenant_ID           TEXT NOT NULL,
    Raw_Pcap_File_Path  TEXT NOT NULL,
    Parsed_Data         JSONB NOT NULL,

    PRIMARY KEY (Pcap_ID),

    FOREIGN KEY (Network_ID) REFERENCES Networks(Network_ID)
);

CREATE INDEX binary_data_index ON Traffic USING gin(Parsed_Data);
