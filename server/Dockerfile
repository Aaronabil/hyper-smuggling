# Stage 1: Build the application
FROM rust:1.70 as builder

WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock to cache dependencies
COPY Cargo.toml ./
# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main(){}" > src/main.rs
# Build only dependencies to cache them
RUN cargo build --release

# Copy the actual source code
COPY src ./src
# Build the application
RUN touch src/main.rs && cargo build --release

# Stage 2: Create the final, smaller image
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/server /usr/local/bin/server
# Copy HTML files
COPY src/index.html /usr/src/app/src/index.html
COPY src/secret.html /usr/src/app/src/secret.html

WORKDIR /usr/src/app

# Command to run the binary
CMD ["/usr/local/bin/server"]