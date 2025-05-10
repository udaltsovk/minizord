ARG CRATE

FROM lukemathwalker/cargo-chef:0.1.71-rust-1.86-alpine3.20 AS chef
WORKDIR /usr/src/minizord

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 

ARG CRATE
ENV PKG_CONFIG_ALLOW_CROSS=1 \
    OPENSSL_STATIC=1

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    git

COPY --from=planner /usr/src/minizord/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin $CRATE

COPY . .
RUN cargo build --release --bin $CRATE


FROM alpine:3.21

ARG CRATE
ENV BIN=$CRATE

RUN apk add --no-cache ca-certificates

WORKDIR /minizord

RUN adduser -DH minizord
USER minizord

COPY --from=builder /usr/src/minizord/target/release/$CRATE $CRATE

CMD /minizord/$BIN
