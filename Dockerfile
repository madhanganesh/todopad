# Build stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

# Install dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    curl \
    tar && \
    apt-get clean

# Install Litestream
RUN curl -L https://github.com/benbjohnson/litestream/releases/download/v0.3.13/litestream-v0.3.13-linux-amd64.tar.gz | tar xz && \
    mv litestream /usr/local/bin/

# Set up working directory
WORKDIR /app

# Copy the built Rust binary
COPY --from=builder /app/target/release/todopad /app/todopad

# Copy necessary assets
COPY migrations /app/migrations
COPY static /app/static

# Copy Litestream config file (Ensure this exists in your project)
COPY litestream.yml /etc/litestream.yml

EXPOSE 8080

# Start Litestream and then run your Rust app
CMD [ "sh", "-c", "/usr/local/bin/litestream replicate -config /etc/litestream.yml & exec /app/todopad" ]
