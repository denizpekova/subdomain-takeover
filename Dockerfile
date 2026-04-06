# === Build Stage ===
FROM rust:1.77-slim AS builder

WORKDIR /app

# Copy manifests to pre-build dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies and cache them
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# Copy real source code
COPY src ./src

# Touch main.rs to force recompilation of the application itself 
RUN touch src/main.rs \
    && cargo build --release

# === Runtime Stage ===
FROM debian:bookworm-slim

WORKDIR /app

# Install required SSL/TLS certificates and dependencies for reqwest
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/subdomain-takeover /usr/local/bin/

# Set the binary as the entrypoint
ENTRYPOINT ["subdomain-takeover"]
