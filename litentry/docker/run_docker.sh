#!/usr/bin/env bash
docker run -d -p 9944:9944 -p 9933:9933 -p 30333:30333 -v /data/substrate:/substrate/data/ \
-v /data/bin:/substrate/bin litentry



