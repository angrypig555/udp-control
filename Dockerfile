FROM rust:slim-trixie as builder

WORKDIR /usr/src/udp-control
COPY . .
RUN cargo install --path .

FROM debian:trixie-slim
COPY --from=builder /usr/local/cargo/bin/udp-control /usr/local/bin/udp-control
CMD ["udp-control"]