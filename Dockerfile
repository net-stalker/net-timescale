# Dockerfile is relevant for monorepo
# Dockerfile is generic for any projects like net-agent and net-hub.
# To build dicker image for spesific project you need to set right name of the project.
FROM rust as base

RUN apt-get update
RUN apt-get install -y tcpdump netcat libpcap-dev libzmq3-dev build-essential cmake
# libpq-dev is used by net-timescale
RUN apt-get install libpq-dev
# Set environment variable PG_CONFIG: Ensure that the environment variable `PG_CONFIG` is set to the directory where `pg_config` is located. On Linux, `pg_config` is usually located in the "/usr/bin" directory. To set the environment variable, you can run the command `export PG_CONFIG=/usr/bin/pg_config`.
RUN export PG_CONFIG=/usr/bin/pg_config
# install nng https://launchpad.net/ubuntu/+source/nng
RUN git clone https://github.com/nanomsg/nng.git && cd nng && mkdir build && cd build && cmake .. && make && make install

# This will install capnp, the Cap’n Proto command-line tool. It will also install libcapnp, libcapnpc, and libkj in /usr/local/lib and headers in /usr/local/include/capnp and /usr/local/include/kj.
RUN curl -O https://capnproto.org/capnproto-c++-0.10.4.tar.gz && tar zxf capnproto-c++-0.10.4.tar.gz && cd capnproto-c++-0.10.4 && ./configure && make -j6 check && make install


FROM base as build

ARG PROJ_NAME

WORKDIR /usr/src/net-monitor

COPY net-core net-core
COPY ./${PROJ_NAME}/Cargo.toml ${PROJ_NAME}/Cargo.toml
COPY ./${PROJ_NAME}/src ${PROJ_NAME}/src

WORKDIR /usr/src/net-monitor/${PROJ_NAME}
RUN cargo build

FROM build as starter

#Copy configuration to config
COPY ./${PROJ_NAME}/.config/application.conf /root/.config/${PROJ_NAME}/application.conf

ENV PROJ_NAME ${PROJ_NAME}

CMD "target/debug/$PROJ_NAME"
