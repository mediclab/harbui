FROM rust:alpine as builder

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk --no-cache add --update pkgconfig musl-dev openssl-dev clang-dev build-base make ca-certificates npm \
    && rm -rf /var/cache/apk/*

WORKDIR /app

COPY . /app

RUN cd ./resources && npm install && npm run prod
RUN cd /app && cargo build --release

FROM alpine:latest

ARG HARBUI_VERSION=dev
ENV HARBUI_VERSION=$HARBUI_VERSION
ENV RUST_LOG=warn

MAINTAINER mediclab
LABEL authors="mediclab"
LABEL org.opencontainers.image.authors="mediclab <m@mdlb.cc>"
LABEL version=$HARBUI_VERSION
LABEL description="Docker Registry UI"

WORKDIR /app

RUN apk --no-cache add --update ca-certificates openssl libgcc libstdc++ \
    && rm -rf /var/cache/apk/*

COPY ./templates /app/templates
COPY ./Rocket.toml /app/Rocket.toml
COPY --from=builder /app/target/release/harbui /app
COPY --from=builder /app/public /app/public

CMD ["/app/harbui"]