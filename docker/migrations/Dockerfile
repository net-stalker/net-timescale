# Stage 1: Build stage using Rust image
FROM rust:latest as builder

# Install sqlx-cli
RUN cargo install sqlx-cli

# Copy the migration files to the builder stage
COPY migrations ./migrations

# Set the entrypoint for running migrations
ENTRYPOINT ["sh", "-c", "sqlx migrate run --database-url $DATABASE_URL"]