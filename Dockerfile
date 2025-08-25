# Build stage
FROM rust:1.89.0-alpine3.22 AS builder

# Устанавливаем зависимости для сборки
RUN apk add --no-cache musl-dev openssl-dev pkgconfig openssl-libs-static

WORKDIR /app

# Копируем файлы зависимостей для кэширования
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Копируем реальный код и пересобираем
COPY src ./src
COPY templates ./templates
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM alpine:3.22

# Устанавливаем runtime зависимости
RUN apk add --no-cache libgcc openssl ca-certificates

WORKDIR /app

# Копируем бинарник
COPY --from=builder /app/target/release/webaggregator /app/

# Даем права на выполнение
RUN chmod +x /app/webaggregator

# Создаем пользователя для безопасности
RUN adduser -D -u 1000 myuser
USER myuser

COPY static ./static

# Токен будет передан через .env файл
CMD ["/app/webaggregator"]