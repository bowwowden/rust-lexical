# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake clang

## Add source code to the build stage.
ADD . /rust-lexical
WORKDIR /rust-lexical/fuzz 

## TODO: ADD YOUR BUILD INSTRUCTIONS HERE.
RUN cargo  +nightly rustc \
    --bin "parse-integer-i8"\
    --bin "parse-integer-i16" \
    --\
    -C passes='sancov-module' \
    -C llvm-args='-sanitizer-coverage-level=3' \
    -C llvm-args='-sanitizer-coverage-inline-8bit-counters' \
    -Z sanitizer=address

# Package Stage
FROM --platform=linux/amd64 ubuntu:20.04

## TODO: Change <Path in Builder Stage>
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-i8 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-i6 /
