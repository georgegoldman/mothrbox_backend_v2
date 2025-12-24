# Build stage
FROM rust:latest as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests and source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build for release
RUN cargo build --release

# Verify binary exists
RUN test -f /app/target/release/mothrbox_backend_v2 && echo "✓ Binary built successfully"

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/mothrbox_backend_v2 /app/app

# Make sure it's executable
RUN chmod +x /app/app

# Set environment defaults
ENV PORT=8000

# Expose the port
EXPOSE 8000

# Run the application with full output
CMD ["/bin/sh", "-c", "echo 'Starting Mothrbox Backend...' && echo 'PORT='$PORT && /app/app"]
