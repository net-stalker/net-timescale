--liquibase formatted sql

--changeset net-illia-stetsenko:1
CREATE TABLE realtime_updating_history
(
    connection_id bigint NOT NULL,
    last_used_index bigint NOT NULL,
    PRIMARY KEY (connection_id)
);

--changeset net-illia-stetsenko:2
CREATE INDEX ON realtime_updating_history ( connection_id );