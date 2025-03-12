ARG CRATE

FROM rust:1.84-alpine3.21 AS builder

ARG CRATE
ENV PKG_CONFIG_ALLOW_CROSS=1 \
    OPENSSL_STATIC=1

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static

# Здесь можно было и без этого обойтись, но при разработке кэши очень помогают
WORKDIR /usr/src/ad_platform
COPY ./Cargo.* ./

COPY ./crates/env-config crates/env-config/
COPY ./crates/profanity-checker crates/profanity-checker/
COPY ./crates/surrealdb-helper crates/surrealdb-helper/
COPY ./crates/backend/Cargo.toml crates/backend/Cargo.toml
COPY ./crates/bot/Cargo.toml crates/bot/Cargo.toml

RUN for crate in backend bot; do\
    mkdir crates/$crate/src; \
    echo 'fn main() {}' > crates/$crate/src/main.rs; \
    done
RUN cargo fetch --target "$(uname -m)-unknown-linux-musl"

COPY ./crates/$CRATE crates/$CRATE/
RUN cargo build --release -p $CRATE --target "$(uname -m)-unknown-linux-musl"
RUN cp "target/$(uname -m)-unknown-linux-musl/release/$CRATE" target/release/$CRATE


FROM alpine:3.21

ARG CRATE
ENV BIN=$CRATE

RUN apk add --no-cache ca-certificates

WORKDIR /ad_platform

RUN adduser -DH ad_platform
USER ad_platform

COPY --from=builder /usr/src/ad_platform/target/release/$CRATE $CRATE

CMD /ad_platform/$BIN
