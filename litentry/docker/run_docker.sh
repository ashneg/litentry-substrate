#!/usr/bin/env bash
sudo docker run --name litentry-runtime --network="host" -v /root/data:/root/data/ -d litentry-runtime

