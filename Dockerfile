FROM rust

RUN apt-get update
RUN apt-get install -y tcpdump netcat libpcap-dev libzmq3-dev

WORKDIR /usr/src/monitor

COPY ./Cargo.toml Cargo.toml
COPY ./src src

RUN cargo clean && cargo build

COPY ./.config/application.conf /root/.config/net-monitor/application.conf

CMD ["target/debug/netmonitor"]
