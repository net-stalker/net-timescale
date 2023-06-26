--liquibase formatted sql

--changeset dshcherbatiuk:1
CREATE TABLE captured_traffic
(
    frame_time  TIMESTAMPTZ NOT NULL,
    src_addr    text        NOT null,
    dst_addr    text        NOT null,
    binary_data JSONB       NOT null,
    PRIMARY KEY (frame_time)
);

--changeset dshcherbatiuk:2
CREATE INDEX ON captured_traffic (src_addr, frame_time DESC);

--changeset dshcherbatiuk:3
CREATE INDEX ON captured_traffic (dst_addr, frame_time DESC);

--changeset dshcherbatiuk:4
SELECT create_hypertable('captured_traffic', 'frame_time');