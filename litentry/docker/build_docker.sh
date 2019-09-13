#!/usr/bin/env bash
docker build -t litentry-runtime .
docker tag litentry-runtime juniuszhou/litentry-runtime:latest
docker push juniuszhou/litentry-runtime:latest

