# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /lru-rs/

WORKDIR /lru-rs/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>

COPY --from=builder /lru-rs/fuzz/target/x86_64-unknown-linux-gnu/release/lru_put /

COPY --from=builder /lru-rs/fuzz/target/x86_64-unknown-linux-gnu/release/lru_multi_put /

COPY --from=builder /lru-rs/fuzz/target/x86_64-unknown-linux-gnu/release/lru_put_and_replace /

COPY --from=builder /lru-rs/fuzz/target/x86_64-unknown-linux-gnu/release/demote /