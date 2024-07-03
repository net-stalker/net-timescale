FROM timescale/timescaledb-ha:pg14-latest

# Switch to the root user to ensure we have the necessary permissions
USER root

# Update the package list and install pg_cron
RUN apt-get update && \
    apt-get install -y postgresql-14-cron && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Switch back to the postgres user
USER postgres

# Copy initialization scripts
COPY scripts/create_user.sql /docker-entrypoint-initdb.d/

# Set the entrypoint to initialize the database and preload pg_cron
ENTRYPOINT ["docker-entrypoint.sh", "postgres", "-c", "shared_preload_libraries=timescaledb,pg_cron"]
