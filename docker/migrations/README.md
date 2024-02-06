# Establish team changelog standards

https://www.liquibase.org/get-started/best-practices

## One change per changeset

We strongly encourage having **only one change per changeset**. This makes each change atomic within
a
single transaction. Each changeset either succeeds or fails. If it fails, it can be corrected and
redeployed until it succeeds. Multiple independent changes in one changeset create a risk that some
changes deploy while a later change fails. This leaves the database in a partially deployed state
which requires manual intervention to correct.

The exception is when you have several changes you want to be grouped as a single transaction – in
that case, multiple statements in the changeset if the correct choice.

## Define the team’s changeset ID format

Choose a changeset ID that works for you. While it can be any string, we advise using an increasing
number sequence starting at 1. Remember that each **changeset ID needs to be unique within the
changelog**.

## Define the team’s changelog filename format

The changelog filename format should be an increasing number sequence starting at 001. Remember that
each **changelog filename needs to be unique within the one release**.

## One directory for every release

For every release will be created changelog-x.y.z.xml and saved
to [changelog](liquibase%2Fchangelog).
All scripts related to the current release will be saved to the appropreate directory in the script
directory.

For instance, current is release 1.0.0. We need to
create [changelog-1.0.0.xml](liquibase%2Fchangelog%2Fchangelog-1.0.0.xml)[changelog](liquibase%2Fchangelog)
and in th directory [scripts](liquibase%2Fchangelog%2Fscripts) need to
create [1.0.0](liquibase%2Fchangelog%2Fscripts%2F1.0.0).

## Document unclear or complicated changesets

Most of the time, changesets are self-documenting.

However, remember to use <comments> for any changesets where you need to explain non-obvious or
complicated database changes to other developers.

## Have a rollback plan

Write changesets so they work with Liquibase rollback.

1. Use a relevant Liquibase change type instead of using a custom <sql> tag.
2. Include a Liquibase rollback tag (<rollback>) whenever a change doesn’t support an out-of-box
   rollback. (e.g., <sql>, <insert>, etc).

Make sure to test rollbacks in development to ensure the production rollback is safe and
predictable.

## Manage your reference data

Leverage Liquibase to manage your reference data. Environment separation (DEV, QA, PROD) can be
achieved using Liquibase contexts. This functionality is helpful in the following situations:

1. When you have test data that should only get deployed to QA environments
2. When managing application configuration data – country table, application configuration data, etc
3. When deplying data-fixes specific to the pre-production and production environments


