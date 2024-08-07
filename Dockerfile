FROM rust:slim-bookworm

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

RUN mv target/release/rlog .

CMD ["./rlog"]
