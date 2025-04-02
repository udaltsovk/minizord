#! /usr/bin/env bash
RUSTFLAGS="-Z threads=16 -Z macro-backtrace" watchexec -qc reset "surrealdb-migrations apply && cargo run -p api"
