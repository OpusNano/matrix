# Build stage
FROM rust:1.88-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy project files
COPY server/Cargo.toml ./
COPY server/src ./src
COPY static ./static
COPY content ./content

# Generate lockfile if not exists
RUN if [ ! -f Cargo.lock ]; then cargo generate-lockfile; fi

# Build release binary
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim AS production

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r appgroup && useradd -r -g appgroup appuser

# Copy binary and static files
COPY --from=builder /app/target/release/matrix-blog /app/
COPY --from=builder /app/static /app/static
COPY --from=builder /app/content /app/content

# Change ownership
RUN chown -R appuser:appgroup /app

USER appuser

EXPOSE 8080

CMD ["/app/matrix-blog"]
