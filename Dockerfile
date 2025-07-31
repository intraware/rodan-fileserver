FROM rust:1.88.0-alpine3.22 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN apk add --no-cache musl-dev && cargo fetch
COPY . .
RUN cargo b --release

FROM alpine:latest AS runner
WORKDIR /app
COPY --from=builder /app/target/release/rodan-fileserver ./rodan-fileserver
CMD ["./rodan-fileserver"]
