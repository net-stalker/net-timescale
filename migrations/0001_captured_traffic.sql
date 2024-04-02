CREATE TABLE Networks
(
    NetworkID           SERIAL,
    NetworkName         TEXT NOT NULL,
    TenantId            TEXT NOT NULL,
    NetworkColor        INT,

    PRIMARY KEY (NetworkID)
    
    UNIQUE (NetworkName, TenantId)
);

CREATE TABLE Traffic
(
    PcapID              SERIAL,
    InsertionTime       TIMESTAMPTZ NOT NULL,
    NetworkID           INT,
    TenantId            TEXT NOT NULL,
    RawPcapFileAddress  TEXT NOT NULL,
    ParsedData          JSONB NOT NULL,

    PRIMARY KEY (PcapID)

    FOREIGN KEY (NetworkID) REFERENCES Networks(NetworkID)
);

CREATE INDEX binary_data_index ON Traffic USING gin(ParsedData);
