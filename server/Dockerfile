FROM rust:1.70

WORKDIR /app
COPY Cargo.toml .
COPY src/ src/

RUN cargo build --release

EXPOSE 3000
CMD ["./target/release/hyper-server"]