FROM timescale/timescaledb-ha:pg14-latest

COPY ./certs/server.key /var/lib/postgresql
COPY ./certs/server.crt /var/lib/postgresql

COPY ./certs/root.crt /var/lib/postgresql
# COPY ./certs/out/myCA.crl /var/lib/postgresql

COPY set-ssl-conf.sh /usr/local/bin/
COPY create_user.sql /docker-entrypoint-initdb.d/



USER root

RUN chown postgres:postgres /var/lib/postgresql/server.crt /var/lib/postgresql/server.key /var/lib/postgresql/root.crt  && \
    chmod 600 /var/lib/postgresql/server.crt /var/lib/postgresql/server.key /var/lib/postgresql/root.crt

USER postgres

ENTRYPOINT ["docker-entrypoint.sh"]

CMD ["postgres", "-c", "ssl=on", "-c", "ssl_cert_file=/var/lib/postgresql/server.crt", "-c", "ssl_key_file=/var/lib/postgresql/server.key",\
    "-c", "ssl_ca_file=/var/lib/postgresql/root.crt"]