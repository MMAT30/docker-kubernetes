FROM --platform=x86_64 rust:latest AS planner
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json



FROM --platform=x86_64 rust:latest AS cacher
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-chef
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json



FROM --platform=x86_64 rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --target x86_64-unknown-linux-musl



FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/multi-stage /multi-stage
CMD ["/multi-stage"]