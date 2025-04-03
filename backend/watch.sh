#! /usr/bin/env bash
RUSTFLAGS="-Z threads=16 -Z macro-backtrace" watchexec -rqc reset -e rs,surql "surrealdb-migrations apply && cargo run -p api"
