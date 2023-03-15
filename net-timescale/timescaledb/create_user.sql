-- Create a liquibase with a password:
CREATE USER liquibase WITH ENCRYPTED PASSWORD 'PsWDgxZb';
-- Grant ALL PRIVILEGES permissions on the database schema to the liquibase
GRANT ALL PRIVILEGES ON SCHEMA public TO liquibase;
-- Grant USAGE permission on the database schema to the liquibase
GRANT USAGE ON SCHEMA public TO liquibase;

-- Create a netuser with a password:
CREATE USER netuser WITH ENCRYPTED PASSWORD 'PsWDgxZb';
-- Grant ALL PRIVILEGES permissions on the database schema to the netuser
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO netuser;
-- Grant USAGE permission on the database schema to the netuser
GRANT USAGE ON SCHEMA public TO netuser;