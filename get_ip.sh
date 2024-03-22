#!/bin/bash

# Get the network name
NETWORK=$(docker network ls --filter name='_' --format '{{ .Name }}')

# Get the IP address of the specified hostname
IP=$(docker network inspect $NETWORK --format '{{range .Containers}}{{.Name}}{{"\t"}}{{.IPv4Address}}{{"\n"}}{{end}}' | grep $1 | awk '{print $2}')

# Output the IP address
echo $IP