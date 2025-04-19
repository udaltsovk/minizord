#! /usr/bin/env bash
watchexec \
    -rqc reset \
    -e surql "
    surrealdb-migrations apply --config-file ./crates/$1/repository/db/surreal/.surrealdb.toml
    "
