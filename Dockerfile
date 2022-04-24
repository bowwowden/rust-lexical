FROM ubuntu:20.04 as builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake clang curl
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN ${HOME}/.cargo/bin/rustup default nightly
RUN ${HOME}/.cargo/bin/cargo install -f cargo-fuzz

## Add source code to the build stage.
# ADD . /rust-lexical
# WORKDIR /rust-lexical/fuzz 
ADD . /repo
WORKDIR /repo

## TODO: ADD YOUR BUILD INSTRUCTIONS HERE.
RUN cd fuzz && ${HOME}/.cargo/bin/cargo fuzz build

# Package Stage
FROM ubuntu:20.04

## TODO: Change <Path in Builder Stage>
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-i8 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-i16 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-i32 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-i64 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-i128 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-isize /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-u8 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-u16 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-u32 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-u64 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-u128 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-integer-usize /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-float-f32 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/parse-float-f64 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-float-f32 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-float-f64 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-i8 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-i16 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-i32 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-i64 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-i128 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-isize /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-u8 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-u16 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-u32 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-u64 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-u128 /
COPY --from=builder repo/fuzz/target/x86_64-unknown-linux-gnu/release/write-integer-usize /

























