FROM rust:slim-buster as build

RUN USER=root cargo new --bin currc
WORKDIR /currc

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./cache.json ./cache.json 

RUN apt-get update && apt install -y libssl-dev pkg-config
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/currc*
RUN cargo build --release

FROM rust:1.49

COPY --from=build /currc/target/release/currc .
COPY --from=build /currc/cache.json .
RUN chmod +x ./currc
CMD ["./currc help"]

