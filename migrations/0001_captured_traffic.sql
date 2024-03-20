CREATE TABLE captured_traffic
(
    id SERIAL,
    frame_time  TIMESTAMPTZ NOT NULL,
    group_id   text        NOT NULL,
    agent_id    text        NOT NULL,
    src_addr    text        NOT null,
    dst_addr    text        NOT null,
    binary_data JSONB       NOT null,
    PRIMARY KEY (frame_time, group_id, agent_id)
);

CREATE INDEX ON captured_traffic (src_addr, frame_time DESC);

CREATE INDEX ON captured_traffic (dst_addr, frame_time DESC);

CREATE INDEX binary_data_index ON captured_traffic USING gin(binary_data);

SELECT create_hypertable('captured_traffic', 'frame_time');
