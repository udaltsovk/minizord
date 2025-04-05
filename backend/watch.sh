#! /usr/bin/env bash
RUSTFLAGS="-Z threads=16 -Z macro-backtrace" watchexec -rqc reset -e rs,toml,surql "cargo fmt --all && surrealdb-migrations apply && cargo clippy --all && cargo run -p api"
