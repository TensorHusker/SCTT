# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy workspace files
COPY rust/Cargo.toml rust/Cargo.lock ./
COPY rust/sctt-core ./sctt-core
COPY rust/sctt-smooth ./sctt-smooth
COPY rust/sctt-cubical ./sctt-cubical
COPY rust/sctt-checker ./sctt-checker
COPY rust/sctt-server ./sctt-server

# Build release binary
RUN cargo build --release --bin sctt-server

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/sctt-server /app/sctt-server

# Copy static files and content
COPY static ./static
COPY content ./content

# Create data directories
RUN mkdir -p data/runetika data/libertalia

EXPOSE 8080

CMD ["./sctt-server"]