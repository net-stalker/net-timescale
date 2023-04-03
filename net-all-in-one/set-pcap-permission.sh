#!/bin/sh
setcap cap_net_raw,cap_net_admin=eip ../target/debug/net-all-in-one
