FROM rust:1.40 as builder


# COPY config /usr/local/cargo/bin/cargo/
COPY config /usr/local/cargo/
RUN USER=root cargo new --bin swarm-sshfs
WORKDIR ./swarm-sshfs
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/swarm_sshfs*
RUN cargo build --release


# COPY Cargo.toml Cargo.toml
# RUN mkdir src/
# RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# # build all the dep packages
# RUN cargo install --verbose --path .
# # RUN cargo build --release
# # now just remove my package from there and rebuild it when the new file copied
# # RUN rm -f target/release/deps/myapp*

# COPY ./src .
# RUN cargo install --path .

# RUN cargo build --release


FROM debian:buster
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /swarm-sshfs/target/release/swarm-sshfs /usr/local/bin/myapp
# CMD ["myapp"]