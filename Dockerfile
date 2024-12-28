# Build stage
FROM rust:bookworm AS builder
 
WORKDIR /app
COPY . .
RUN cargo build --release
 
# Final run stage
FROM debian:bookworm-slim AS runner
 
WORKDIR /app
COPY --from=builder /app/target/release/todopad /app/todopad
COPY migrations /app/migrations
COPY seeds /app/seeds
COPY static /app/static
COPY .env /app/.env

EXPOSE 3000
CMD ["/app/todopad"]
