#!/usr/bin/env bash
cargo build --release
eval $(cat .env) /tmp/target/release/server
