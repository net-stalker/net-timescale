FROM debian:bullseye-slim AS builder

# Install required packages
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    libzmq3-dev \
    libpcap-dev \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN wget https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init && \
    chmod +x rustup-init && \
    ./rustup-init -y && \
    rm rustup-init

ENV PATH="/root/.cargo/bin:${PATH}"

# Create a new directory for the application
WORKDIR /net-timescale

# Copy the Rust source code
COPY . .

# Build the application
RUN cargo build --package net-reporter --release

# Final stage
FROM debian:bullseye-slim
COPY --from=builder /net-timescale/target/release/net-reporter /
COPY --from=builder /net-timescale/net-reporter/config.toml /
COPY --from=builder /net-timescale/migrations /migrations

ENV CONFIG_PATH=/

ENTRYPOINT ["/net-reporter"]