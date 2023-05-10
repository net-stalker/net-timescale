FROM timescale/timescaledb-ha:pg14-latest

# Copy the SSL certificate and key files to the container
COPY .ssl/host.cert /.ssl/
COPY .ssl/host.key /.ssl/

# Create the data directory and set ownership and permissions
RUN mkdir -p /var/lib/postgresql/data && \
    chown -R postgres:postgres /var/lib/postgresql && \
    chmod 700 /var/lib/postgresql/data

# Enable SSL for the PostgreSQL listener
RUN echo "ssl = on" >> /var/lib/postgresql/data/postgresql.conf && \
    echo "ssl_cert_file = '/.ssl/host.cert'" >> /var/lib/postgresql/data/postgresql.conf && \
    echo "ssl_key_file = '/.ssl/host.key'" >> /var/lib/postgresql/data/postgresql.conf

# Disable the plain text listener
RUN echo "hostssl all all 0.0.0.0/5432 md5" >> /var/lib/postgresql/data/pg_hba.conf && \
    echo "host all all 0.0.0.0/5432 reject" >> /var/lib/postgresql/data/pg_hba.conf

# Copy the create_user.sql script to the container's initdb directory
COPY create_user.sql /docker-entrypoint-initdb.d/

CMD ["postgres"]
