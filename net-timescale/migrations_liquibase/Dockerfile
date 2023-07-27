FROM liquibase/liquibase:latest

COPY liquibase/changelog.xml /liquibase/changelog.xml
COPY liquibase/changelog/ /liquibase/changelog/

COPY liquibase.properties /liquibase/liquibase.properties

#You can also customize the Liquibase command by passing arguments to the CMD directive in the Dockerfile. For example, if you want to specify a different Liquibase command (such as rollback) and a different changelog file, you can use the following CMD:
#This will run Liquibase with the rollbackCount command and a changelog file located at /liquibase/my-changelog.xml.
#CMD ["liquibase", "--changeLogFile=/liquibase/my-changelog.xml", "rollbackCount", "1"]
CMD ["sh", "-c", "liquibase clear-checksums && liquibase update"]