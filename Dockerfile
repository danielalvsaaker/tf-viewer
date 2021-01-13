FROM rust:alpine as builder
RUN apk add --no-cache make tzdata musl-dev

COPY . .
RUN cargo install --path . --features jemalloc

FROM alpine:3.12
RUN apk add --no-cache tzdata
COPY --from=builder /usr/local/cargo/bin/tf-viewer /bin/tf-viewer

WORKDIR /data
VOLUME ["/data"]
EXPOSE 8080
ENTRYPOINT ["/bin/tf-viewer"]
