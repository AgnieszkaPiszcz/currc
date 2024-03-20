FROM rust:latest

COPY ./ ./

RUN cargo build

ENTRYPOINT ["./target/debug/currc"]

CMD ["help"]

