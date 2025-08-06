FROM rust:1.88.0-alpine3.22 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock .
COPY src ./src
RUN mkdir -p data
RUN apk add --no-cache musl-dev && cargo fetch
RUN cargo b --release

FROM alpine:latest AS runner
WORKDIR /app
COPY --from=builder /app/target/release/rodan-fileserver ./rodan-fileserver
COPY static ./static
CMD ["./rodan-fileserver"]
