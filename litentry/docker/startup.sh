#!/usr/bin/env bash
RUST_LOG=debug RUST_BACKTRACE=1 /usr/local/bin/litentry-substrate --dev --rpc-cors=all \
--ws-external=true --rpc-external=true --base-path=/substrate/data/litentry
