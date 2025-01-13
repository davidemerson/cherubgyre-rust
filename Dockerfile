# Build stage
FROM rust:1.84-alpine AS builder

# Install necessary tools using apk (Alpine package manager)
RUN apk add --no-cache git ca-certificates

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo files first to leverage caching for dependencies
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies (cached unless dependencies change)
RUN cargo fetch

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Runtime stage
FROM alpine:latest

# Install SSL certificates using apk
RUN apk add --no-cache ca-certificates

# Install musl-dev for building musl-based Rust projects
RUN apk add --no-cache musl-dev gcc

# Set the working directory
WORKDIR /usr/src/app

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/app/target/release/cherubgyre .

# Expose the API port
EXPOSE 8080

# Command to run the API
CMD ["./cherubgyre"]
