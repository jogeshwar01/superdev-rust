FROM rust:1.88 AS builder

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/superdev /usr/local/bin/superdev
EXPOSE 8080
CMD ["superdev"] 