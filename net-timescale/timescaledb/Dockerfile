FROM timescale/timescaledb-ha:pg14-latest

COPY ./certs/server.key /var/lib/postgresql
COPY ./certs/server.crt /var/lib/postgresql
# COPY create_user.sql /docker-entrypoint-initdb.d/

USER root

RUN chown postgres:postgres /var/lib/postgresql/server.crt /var/lib/postgresql/server.key && \
    chmod 600 /var/lib/postgresql/server.crt /var/lib/postgresql/server.key

# Copy the custom entrypoint script
COPY entrypoint.sh /entrypoint.sh

USER postgres

ENTRYPOINT ["/entrypoint.sh"]

CMD ["postgres"]