FROM rust:1.55 as builder

# create empty projects
RUN USER=root cargo new --bin common
RUN USER=root cargo new --bin server

# copy the manifests
COPY ./common/Cargo.toml ./common/Cargo.toml
COPY ./server/Cargo.toml ./server/Cargo.toml

# build the dependencies
RUN cd server && cargo build --release
RUN rm common/src/*.rs
RUN rm server/src/*.rs

# copy our source code to the projects
COPY ./common/src ./common/src
COPY ./server/src ./server/src
COPY ./server/words_of_wisdom.txt ./server/words_of_wisdom.txt

# build
WORKDIR server
RUN rm ./target/release/deps/server*
RUN cargo build --release

# final image
FROM debian:bullseye-slim

# copy the build artifacts from the build phase
COPY --from=builder /server/target/release/server .
COPY --from=builder /server/words_of_wisdom.txt .

# set the startup command
CMD ["./server"]
