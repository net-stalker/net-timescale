FROM timescale/timescaledb-ha:pg14-latest

COPY ./certs/ /var/lib/postgresql

COPY scripts/set-ssl-conf.sh /usr/local/bin/
COPY scripts/create_user.sql /docker-entrypoint-initdb.d/

USER root

RUN chown postgres:postgres /var/lib/postgresql/* && \
    chmod 600 /var/lib/postgresql/*

USER postgres

ENTRYPOINT ["docker-entrypoint.sh"]

CMD ["postgres", "-c", "ssl=on", "-c", "ssl_cert_file=/var/lib/postgresql/server.crt", "-c", "ssl_key_file=/var/lib/postgresql/server.key"]