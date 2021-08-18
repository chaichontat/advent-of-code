FROM rust:latest

ENTRYPOINT ["sh", "-c", "cargo test"]