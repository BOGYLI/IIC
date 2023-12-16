FROM rust:1.74-buster as chef
RUN cargo install cargo-chef
WORKDIR /app




FROM chef as diesel
RUN apt-get install libpq-dev -y
RUN cargo install diesel_cli --no-default-features --features postgres
COPY ../../server/migrations ./migrations
RUN diesel setup
RUN diesel migration run




FROM chef AS planner
COPY ../../server .
RUN cargo chef prepare --recipe-path recipe.json




FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY ../../server .
COPY --from=diesel /app/src/db/schema.rs /app/src/db/schema.rs
RUN cargo build --release --bin app




# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get install libpq-dev -y
WORKDIR /app
COPY --from=builder /app/target/release/app /usr/local/bin
ENTRYPOINT ["/usr/local/bin/app"]