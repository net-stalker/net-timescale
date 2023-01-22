# Dockerfile is relevant for monorepo
# Dockerfile is generic for any projects like net-agent and net-hub.
# To build dicker image for spesific project you need to set right name of the project.
FROM rust as build

RUN apt-get update
RUN apt-get install -y tcpdump netcat libpcap-dev libzmq3-dev

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
