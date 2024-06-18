# Stage 1: Build stage using Rust image
FROM rust:latest as builder

# Install sqlx-cli
RUN cargo install sqlx-cli

# Install PostgreSQL client tools
RUN apt-get update && apt-get install -y postgresql-client

# Copy the migration files to the builder stage
COPY migrations ./migrations

# Set the entrypoint for running migrations
ENTRYPOINT ["sh", "-c", "until pg_isready -h timescaledb -U postgres; do sleep 1; done && sqlx database create --database-url $DATABASE_URL && sqlx migrate run --database-url $DATABASE_URL"]
