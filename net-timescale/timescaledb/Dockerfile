FROM timescale/timescaledb-ha:pg14-latest

COPY create_user.sql /docker-entrypoint-initdb.d/

CMD ["postgres"]
