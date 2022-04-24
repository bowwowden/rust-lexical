# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake clang curl
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN ${HOME}/.cargo/bin/rustup default nightly
RUN ${HOME}/.cargo/bin/cargo install -f cargo-fuzz

## Add source code to the build stage.
ADD . /rust-lexical
WORKDIR /rust-lexical/fuzz 

## TODO: ADD YOUR BUILD INSTRUCTIONS HERE.
RUN cd fuzz && ${HOME}/.cargo/bin/cargo fuzz build
# RUN cargo  +nightly rustc \
#     --\
#     -C passes='sancov-module' \
#     -C llvm-args='-sanitizer-coverage-level=3' \
#     -C llvm-args='-sanitizer-coverage-inline-8bit-counters' \
#     -Z sanitizer=address

# Package Stage
FROM --platform=linux/amd64 ubuntu:20.04

## TODO: Change <Path in Builder Stage>
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-i8 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-i6 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-i32 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-i64 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-i128 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-isize /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-u8 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-u16 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-u32 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-u64 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-u128 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-integer-usize /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-float-f32 /
COPY --from=builder /rust-lexical/fuzz/target/debug/parse-float-f64 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-float-f32 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-float-f64 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-i8 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-i16 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-i32 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-i64 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-i128 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-isize /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-u8 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-u16 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-u32 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-u64 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-u128 /
COPY --from=builder /rust-lexical/fuzz/target/debug/write-integer-usize /

























