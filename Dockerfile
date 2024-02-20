FROM rust:alpine as builder

ARG HARBUI_VERSION=dev
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk --no-cache add --update pkgconfig musl-dev openssl-dev clang-dev build-base make ca-certificates npm \
    && rm -rf /var/cache/apk/*

WORKDIR /app

COPY . /app

RUN cd ./resources \
    && corepack enable \
    && sed -ie "s/##HARBUI_VERSION##/$HARBUI_VERSION/g" app.vue \
    && yarn install \
    && yarn generate

RUN cd /app && cargo build --release

FROM alpine:latest

ARG HARBUI_VERSION=dev
ENV HARBUI_VERSION=$HARBUI_VERSION
ENV LOG=warn
ENV RUST_LOG=${LOG}
ENV ROCKET_SECRET_KEY=${SECRET_KEY}

MAINTAINER mediclab
LABEL authors="mediclab"
LABEL org.opencontainers.image.authors="mediclab <m@mdlb.cc>"
LABEL version=$HARBUI_VERSION
LABEL description="Docker Registry UI"

WORKDIR /app

RUN apk --no-cache add --update ca-certificates openssl libgcc libstdc++ \
    && rm -rf /var/cache/apk/*

COPY ./Rocket.toml /app/Rocket.toml
COPY --from=builder /app/target/release/harbui /app
COPY --from=builder /app/resources/.output/public /app/public

CMD ["/app/harbui"]