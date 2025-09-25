#!/usr/bin/env bash
set -eu

cargo build --release
eval $(cat .env) /tmp/target/release/server
