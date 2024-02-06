FROM liquibase/liquibase:latest

COPY liquibase/changelog.xml /liquibase/changelog.xml
COPY liquibase/changelog/ /liquibase/changelog/

COPY liquibase.properties /liquibase/liquibase.properties
COPY check-postgres-status.sh check-postgres-status.sh

# Install PostgreSQL client
USER root
RUN apt-get update && apt-get install -y postgresql-client
RUN chmod +x check-postgres-status.sh
USER liquibase

CMD ["sh", "-c", "./check-postgres-status.sh && liquibase update && liquibase clear-checksums"]