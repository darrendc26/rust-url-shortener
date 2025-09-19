FROM rust:latest as builder

WORKDIR /usr/src/url_shortener

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Build dummy main to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy source code
COPY src ./src
# COPY migrations ./migrations

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/url_shortener/target/release/url_shortener /usr/local/bin/url_shortener

EXPOSE 8080

CMD ["url_shortener"]