# Stage 1: Build
FROM rust:1.91 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release


# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/axumlive .

EXPOSE 8000

CMD ["./axumlive"]