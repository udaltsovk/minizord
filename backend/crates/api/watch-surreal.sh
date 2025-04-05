#! /usr/bin/env bash
watchexec -rqc reset -e surql "surrealdb-migrations apply"
