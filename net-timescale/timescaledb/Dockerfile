FROM timescale/timescaledb-ha:pg14-latest

COPY ./certs/out/postgresdb.key /var/lib/postgresql
COPY ./certs/out/postgresdb.crt /var/lib/postgresql

COPY ./certs/out/myCA.crt /var/lib/postgresql
COPY ./certs/out/myCA.crl /var/lib/postgresql

COPY set-ssl-conf.sh /usr/local/bin/
COPY create_user.sql /docker-entrypoint-initdb.d/



USER root

RUN chown postgres:postgres /var/lib/postgresql/postgresdb.crt /var/lib/postgresql/postgresdb.key /var/lib/postgresql/myCA.crt /var/lib/postgresql/myCA.crl && \
    chmod 600 /var/lib/postgresql/postgresdb.crt /var/lib/postgresql/postgresdb.key /var/lib/postgresql/myCA.crt /var/lib/postgresql/myCA.crl

USER postgres

ENTRYPOINT ["docker-entrypoint.sh"]

CMD ["postgres", "-c", "ssl=on", "-c", "ssl_cert_file=/var/lib/postgresql/postgresdb.crt", "-c", "ssl_key_file=/var/lib/postgresql/postgresdb.key",\
    "-c", "ssl_ca_file=/var/lib/postgresql/myCA.crt", "-c", "ssl_crl_file=/var/lib/postgresql/myCA.crl"]