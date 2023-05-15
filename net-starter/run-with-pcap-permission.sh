#!/bin/sh
# Run this script in case you use linux and have faced with problem `PcapError("socket: Operation not permitted")'`
cargo build
# This command sets necessary permissions for the binary to make it possible to open raw sockets
sudo setcap cap_net_raw,cap_net_admin=eip ../target/debug/net-starter
cargo run
