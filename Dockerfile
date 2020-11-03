FROM rust:latest

WORKDIR /
COPY . .

ENV RUST_LOG=info

RUN cargo install --path .

CMD ["modding"]