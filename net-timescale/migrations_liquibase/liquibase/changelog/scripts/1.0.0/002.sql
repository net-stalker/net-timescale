--liquibase formatted sql

--changeset net-illia-stetsenko:1
CREATE OR REPLACE FUNCTION notify_after_insert()
RETURNS TRIGGER AS '
BEGIN
    PERFORM pg_notify(''insert_channel'', '''');
RETURN NEW;
END;'
LANGUAGE plpgsql;

--changeset net-illia-stetsenko:2
CREATE TRIGGER new_data_trigger
AFTER INSERT ON captured_traffic
FOR EACH ROW
EXECUTE FUNCTION notify_after_insert();