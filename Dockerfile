FROM rust:slim
COPY ./target/release/hetzner-bridge ./target/release/hetzner-bridge
ENTRYPOINT ["./target/release/hetzner-bridge"]