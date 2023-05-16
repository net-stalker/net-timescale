FROM timescale/timescaledb-ha:pg14-latest

# Copy the SSL/TLS certificate files to the container
COPY certs/server.crt /var/lib/postgresql/server.crt
COPY certs/server.key /var/lib/postgresql/server.key

COPY create_user.sql /docker-entrypoint-initdb.d/

USER root

# Update permissions for the certificate files
RUN chown postgres:postgres /var/lib/postgresql/server.crt
RUN chown postgres:postgres /var/lib/postgresql/server.key
RUN chmod 600 /var/lib/postgresql/server.crt
RUN chmod 600 /var/lib/postgresql/server.key
# Modify the PostgreSQL configuration file

CMD ["postgres"]

RUN echo "ssl = on" >> /var/lib/postgresql/postgresql.conf
RUN echo "ssl_cert_file = /var/lib/postgresql/server.crt" >> /var/lib/postgresql/postgresql.conf
RUN echo "ssl_key_file = /var/lib/postgresql/server.key" >> /var/lib/postgresql/postgresql.conf

# # Modify the pg_hba.conf file to require SSL/TLS and client certificate authentication
# RUN echo "hostssl all all 127.0.0.1/32 md5" > /home/postgres/pgdata/data/pg_hba.conf
# RUN echo "hostssl all all ::1/128 md5" >> /home/postgres/pgdata/data/pg_hba.conf
# RUN echo "hostssl all all 0.0.0.0/0 md5" >> /home/postgres/pgdata/data/pg_hba.conf

