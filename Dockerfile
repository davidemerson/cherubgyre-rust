# Use a newer Rust image (1.78.0 or later)
FROM rust:1.78 AS builder

# Install git and SSL certificates
RUN apt-get update && \
    apt-get install -y git ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Create a smaller image with just the compiled binary
FROM debian:buster-slim

# Install SSL certificates on the runtime image
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/app/target/release/cherubgyre .

# Expose the API port
EXPOSE 8080

# Command to run the API
CMD ["./cherubgyre"]


