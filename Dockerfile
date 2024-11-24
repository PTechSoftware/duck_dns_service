# Use an official Rust image for building
FROM rust:1.73 AS builder

# Set a working directory
WORKDIR /app

# Copy Cargo.toml and Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Install dependencies
RUN apt-get update && apt-get install -y build-essential
# Set up Rust build environment
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y


# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal image for the runtime
FROM debian:bookworm-slim
# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl-dev && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN apt-get upgrade libc6
RUN apt-get update && apt-get install -y build-essential
RUN apt-get install htop -y
RUN apt-get install net-tools -y

# Set a working directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/duck_dns_service .
COPY config.json .
RUN touch log.txt
# Expose the port your app listens on
EXPOSE 4443 8080

# Run the application
CMD [ "./duck_dns_service" ]
