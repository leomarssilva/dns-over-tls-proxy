FROM rust:slim-bullseye as builder
RUN apt update && apt install -y librust-openssl-dev libssl-dev
WORKDIR /build

# RUN rustup target add $(uname -m)-unknown-linux-musl
# ### Cache the crates used
# COPY src/cache.rs src/cache.rs
# COPY Cargo.* ./
# RUN cargo build --release  --bin cache
###
COPY . .
RUN CARGO_HTTP_MULTIPLEXING=false cargo build --bin dns

# not to be used in production
FROM rust:slim-bullseye
RUN apt update && apt install -y librust-openssl-dev libssl-dev
# WORKDIR /
EXPOSE 53/udp
EXPOSE 53/tcp
COPY --from=builder /build/target/debug/dns /
CMD [ "/dns" ]