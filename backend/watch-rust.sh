#! /usr/bin/env bash
RUSTFLAGS="-Z macro-backtrace" watchexec -rqc reset -e rs,toml "cargo udeps --all && cargo audit && cargo fmt --all && cargo clippy --all && cargo run -p api"
