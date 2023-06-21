-- Your SQL goes here
CREATE TABLE captured_traffic
(
    frame_time  TIMESTAMPTZ NOT NULL,
    src_addr    text        NOT null,
    dst_addr    text        NOT null,
    binary_data JSONB       NOT null,
    PRIMARY KEY (frame_time)
);

CREATE INDEX ON captured_traffic (src_addr, frame_time DESC);

CREATE INDEX ON captured_traffic (src_addr, frame_time DESC);

SELECT create_hypertable('captured_traffic', 'frame_time');