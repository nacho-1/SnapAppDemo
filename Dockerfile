# STAGE1: Build the binary
FROM rust:1.80-alpine3.20 as builder

# Install build dependencies
RUN apk add --no-cache build-base musl-dev openssl-dev openssl

WORKDIR /app

# Copy over the Cargo.toml first to pre-install deps.
COPY Cargo.toml Cargo.lock ./

# Build and cache the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs

# Copy the actual code files and build the application
COPY src ./src/
# Update the file date
RUN touch src/main.rs
RUN cargo build --release

# STAGE2: create a slim image with the compiled binary
FROM alpine:3.20 as runner
WORKDIR /app
COPY --from=builder /app/target/release/snap_app_demo app

CMD ["./app"]