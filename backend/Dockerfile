ARG CRATE

FROM rust:1.85-alpine3.21 AS builder

ARG CRATE
ENV PKG_CONFIG_ALLOW_CROSS=1 \
    OPENSSL_STATIC=1

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    git

# Здесь можно было и без этого обойтись, но при разработке кэши очень помогают
WORKDIR /usr/src/minizord
COPY ./Cargo.* ./

COPY ./crates crates/

RUN cargo fetch --target "$(uname -m)-unknown-linux-musl"

COPY ./crates/$CRATE crates/$CRATE/
RUN cargo build --release -p $CRATE --target "$(uname -m)-unknown-linux-musl"
RUN cp "target/$(uname -m)-unknown-linux-musl/release/$CRATE" target/release/$CRATE


FROM alpine:3.21

ARG CRATE
ENV BIN=$CRATE

RUN apk add --no-cache ca-certificates

WORKDIR /minizord

RUN adduser -DH minizord
USER minizord

COPY --from=builder /usr/src/minizord/target/release/$CRATE $CRATE

CMD /minizord/$BIN
