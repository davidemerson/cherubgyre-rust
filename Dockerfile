# Build Stage
FROM rust:1.84-alpine AS builder

# Install necessary tools
RUN apk add --no-cache git ca-certificates musl-dev gcc libc-dev

# Set the working directory
WORKDIR /usr/src/app

# Copy Cargo files first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies (cached unless Cargo files change)
RUN cargo fetch

# Copy the source code
COPY . .

# Build the application in release mode for musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Runtime Stage
FROM alpine:latest

# Install CA certificates for HTTPS support
RUN apk add --no-cache ca-certificates

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/cherubgyre .

# Ensure the binary is executable
RUN chmod +x ./cherubgyre

# Expose the application port
EXPOSE 8080

# Define the command to run the application
CMD ["./cherubgyre"]
