# ======================================================================
# Local Stage for development
FROM rust:1.76 as local

WORKDIR /usr/src/api-service

RUN cargo install cargo-watch

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm -f target/debug/deps/api_service*

# Copy the source code and migrations
COPY src /usr/src/api-service/src
COPY migrations /usr/src/api-service/migrations

# run cargo watch
CMD ["cargo", "watch", "-x", "run"]

# ======================================================================
# Builder Stage to compile the application
FROM rust:1.76 as builder

WORKDIR /usr/src/api-service

# Copy the Cargo.toml and Cargo.lock and build a dummy project to cache dependencies
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/api_service*

# Copy the source code and migrations
COPY src /usr/src/api-service/src
COPY migrations /usr/src/api-service/migrations

# Build for release
RUN cargo build --release

# ======================================================================
# Prod Stage for the final production image
FROM debian:bookworm-slim as prod

# Expose the port your API service runs on
EXPOSE 3000

# Update and install necessary packages
RUN apt-get update \
  && apt-get install -y curl libpq5 \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/api-service/target/release/api-service /usr/app/api-service

# Set the startup command to run your binary
CMD ["/usr/app/api-service"]

# ======================================================================
# Builder Stage to compile the application for ARM64
FROM --platform=linux/arm64 rust:1.73 as builder-arm64

WORKDIR /usr/src/api-service

# Add the ARM64 target for cross-compilation
RUN rustup target add aarch64-unknown-linux-gnu

# Copy the Cargo.toml and Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Install cross-compilation tools
RUN apt-get update && apt-get install -y gcc-aarch64-linux-gnu libc6-dev-arm64-cross

# Create a dummy project to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target=aarch64-unknown-linux-gnu
RUN rm -f target/aarch64-unknown-linux-gnu/release/deps/api_service*

# Copy the source code and migrations
COPY src /usr/src/api-service/src
COPY migrations /usr/src/api-service/migrations

# Build for release targeting ARM64
ENV RUSTFLAGS="-C target-cpu=native"
RUN cargo build --release --target=aarch64-unknown-linux-gnu

# ======================================================================
# Prod Stage for the final production image for ARM64
FROM --platform=linux/arm64 debian:bookworm-slim as prod-arm64

EXPOSE 3000

RUN apt-get update \
  && apt-get install -y curl libpq5 \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/app

COPY --from=builder-arm64 /usr/src/api-service/target/aarch64-unknown-linux-gnu/release/api-service /usr/app/api-service

CMD ["/usr/app/api-service"]
