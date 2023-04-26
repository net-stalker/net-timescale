#!/bin/sh
cargo build
sudo setcap cap_net_raw,cap_net_admin=eip ../target/debug/net-all-in-one
cargo run