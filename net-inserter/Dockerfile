FROM messense/rust-musl-cross:x86_64-musl as builder

# Install required packages
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    musl-tools \
    libzmq3-dev \
    libpcap-dev \
    gcc-multilib musl-tools \
    wget \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl
RUN export PKG_CONFIG_SYSROOT_DIR=/usr/include
RUN export OPENSSL_DIR=/usr/include/x86_64-linux-gnu/openssl
RUN ln -s /bin/g++ /bin/musl-g++

RUN wget https://www.tcpdump.org/release/libpcap-1.10.4.tar.gz
RUN tar -xzf libpcap-1.10.4.tar.gz
RUN cd libpcap-1.10.4 \
    && CC=x86_64-linux-gnu-gcc ac_cv_linux_vers=2 ./configure --host=x86_64-linux-gnu --with-pcap=linux \
    && make \
    && make install \
    && echo "Completed"

# Create a new directory for the application
WORKDIR /net-timescale

# Copy the Rust source code
COPY . .

# Build the application
RUN cargo build --package net-inserter --release --target=x86_64-unknown-linux-musl

# Final stage
FROM debian:latest
COPY --from=builder /net-timescale/target/x86_64-unknown-linux-musl/release/net-inserter /
COPY --from=builder /net-timescale/net-inserter/config.toml /
COPY --from=builder /net-timescale/migrations /migrations
ENV CONFIG_PATH=/
ENTRYPOINT ["/net-inserter"]
