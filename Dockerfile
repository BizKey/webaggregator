FROM rust:1.97.1-alpine3.24 AS builder

RUN apk add --no-cache musl-dev pkgconfig

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src ./src
COPY templates ./templates
RUN touch src/main.rs && cargo build --release

FROM alpine:3.24

RUN apk add --no-cache libgcc ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/webaggregator /app/

RUN chmod +x /app/webaggregator

RUN adduser -D -u 1000 myuser
USER myuser

COPY static ./static

CMD ["/app/webaggregator"]