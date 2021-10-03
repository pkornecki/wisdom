FROM rust:1.55 as builder

# create empty projects to cache dependencies
RUN USER=root cargo new --bin common
RUN USER=root cargo new --bin client

# copy the manifests
COPY ./common/Cargo.toml ./common/Cargo.toml
COPY ./client/Cargo.toml ./client/Cargo.toml

# build the dependencies
RUN cd client && cargo build --release
RUN rm common/src/*.rs
RUN rm client/src/*.rs

# copy our source code to the projects
COPY ./common/src ./common/src
COPY ./client/src ./client/src

# build
WORKDIR client
RUN rm ./target/release/deps/client*
RUN cargo build --release

# final image
FROM debian:bullseye-slim

# copy the build artifacts from the build phase
COPY --from=builder /client/target/release/client .

# set the startup command
ENTRYPOINT ["./client"]
