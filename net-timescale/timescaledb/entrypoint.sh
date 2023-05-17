#!/bin/bash

set -e

# Start the PostgreSQL server
/docker-entrypoint.sh postgres &

# Wait for the PostgreSQL server to start
until psql -U postgres -c '\q' >/dev/null 2>&1; do
  sleep 1
done

# Modify the PostgreSQL configuration file
echo "ssl = on" >> /home/postgres/pgdata/data/postgresql.conf
echo "ssl_cert_file = '/var/lib/postgresql/server.crt'" >> /home/postgres/pgdata/data/postgresql.conf
echo "ssl_key_file = '/var/lib/postgresql/server.key'" >> /home/postgres/pgdata/data/postgresql.conf

# Modify the pg_hba.conf file to require SSL/TLS and client certificate authentication
echo "hostssl all all 127.0.0.1/32 cert" > /home/postgres/pgdata/data/pg_hba.conf
echo "hostssl all all ::1/128 cert" >> /home/postgres/pgdata/data/pg_hba.conf
echo "hostssl all all 0.0.0.0/0 cert" >> /home/postgres/pgdata/data/pg_hba.conf

# Restart the PostgreSQL server to apply the new configuration
pg_ctl -D /home/postgres/pgdata/data restart

# Wait for the PostgreSQL server to restart
until psql -U postgres -c '\q' >/dev/null 2>&1; do
  sleep 1
done

# Execute any additional commands or scripts here if needed

# Keep the container running
tail -f /dev/null
