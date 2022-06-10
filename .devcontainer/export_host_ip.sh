#!/bin/sh

export DOCKER_HOST_IP=$(/sbin/ip route | awk '/default/ { print $3 }')
echo -n $DOCKER_HOST_IP > ~/hostip
