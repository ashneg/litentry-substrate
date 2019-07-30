#!/usr/bin/env bash
cp ../../target/release/litentry ./
docker build -t litentry-substrate .
rm litentry



