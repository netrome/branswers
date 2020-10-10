FROM rust:1.47.0

RUN rustup ovveride set nightlt
WORKDIR /usr/src/app
COPY . .

RUN cargo install

CMD ["branswers"]
