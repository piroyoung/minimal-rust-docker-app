FROM rust:1.37.0

COPY ./hello_world /usr/src/hello_world
WORKDIR /usr/src/hello_world

RUN cargo install --path .
CMD ["hello_world"]
