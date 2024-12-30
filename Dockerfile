# Build stage
FROM rust:bookworm AS builder
 
WORKDIR /app
COPY . .
RUN cargo build --release
 
# Final run stage
FROM debian:bookworm-slim AS runner
RUN apt-get update && apt-get install -y libssl3 && apt-get clean
RUN apt-get update && apt-get install -y ca-certificates
 
WORKDIR /app
COPY --from=builder /app/target/release/todopad /app/todopad
COPY migrations /app/migrations
COPY static /app/static

EXPOSE 8080
CMD ["/app/todopad"]
