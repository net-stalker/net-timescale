FROM timescale/timescaledb-ha:pg14-latest as builder

COPY ./certs/server.key /var/lib
COPY ./certs/server.crt /var/lib
COPY create_user.sql /docker-entrypoint-initdb.d/

USER root

# Update permissions for the certificate files
RUN chown postgres:postgres /var/lib/server.crt /var/lib/server.key && \
    chmod 600 /var/lib/server.crt /var/lib/server.key

FROM timescale/timescaledb-ha:pg14-latest

COPY --from=builder /var/lib/server.crt /var/lib/postgresql/server.crt
COPY --from=builder /var/lib/server.key /var/lib/postgresql/server.key
COPY create_user.sql /docker-entrypoint-initdb.d/

# Copy the custom entrypoint script
COPY entrypoint.sh /entrypoint.sh
# RUN chmod +x /entrypoint.sh

# Set the custom entrypoint script as the entrypoint for the container
USER postgres

ENTRYPOINT ["/entrypoint.sh"]

CMD ["postgres"]