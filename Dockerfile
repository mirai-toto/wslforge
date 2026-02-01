# Build stage
FROM rust:1.75 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
RUN cargo fetch

COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/hello .
CMD ["./hello"]
