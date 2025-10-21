FROM rust:1.88.0-alpine3.22 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN apk add --no-cache musl-dev && cargo fetch
COPY src ./src
RUN cargo b --release

FROM gcr.io/distroless/cc AS runner
WORKDIR /root
COPY --from=builder /app/target/release/rodan-fileserver .
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
CMD ["/root/rodan-fileserver"]
