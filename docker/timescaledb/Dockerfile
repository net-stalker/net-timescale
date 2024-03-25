FROM timescale/timescaledb-ha:pg14-latest

COPY scripts/create_user.sql /docker-entrypoint-initdb.d/

ENTRYPOINT ["docker-entrypoint.sh"]

CMD ["postgres"]