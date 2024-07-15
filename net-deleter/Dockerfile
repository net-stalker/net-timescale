FROM debian:bullseye-slim AS builder

# Install required packages
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    libzmq3-dev \
    libpcap-dev \
    wget && rm -rf /var/lib/apt/lists/*

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
RUN cargo build --package net-deleter --release

# Final stage
FROM debian:bullseye-slim
RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y tshark

COPY --from=builder /net-timescale/target/release/net-deleter /
COPY --from=builder /net-timescale/net-deleter/config.toml /

ENV CONFIG_PATH=/

ENTRYPOINT ["/net-deleter"]