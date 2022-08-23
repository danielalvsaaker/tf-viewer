FROM rust:alpine AS builder
RUN apk add --no-cache build-base tzdata

WORKDIR /build
COPY . .

RUN cargo install --path . --root /build

FROM alpine:latest
RUN apk add --no-cache tzdata
COPY --from=builder /build/bin/tf-viewer /bin/tf-viewer

WORKDIR /data
VOLUME ["/data"]
EXPOSE 8080
ENTRYPOINT ["/bin/tf-viewer"]
