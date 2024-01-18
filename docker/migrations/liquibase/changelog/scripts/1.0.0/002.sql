--liquibase formatted sql

--changeset net-illia-stetsenko:1
CREATE INDEX binary_data_index ON captured_traffic USING gin(binary_data);