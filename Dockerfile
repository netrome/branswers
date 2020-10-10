FROM rust:1.47.0

RUN rustup override set nightly
WORKDIR /app
COPY . .

RUN cargo install --path . --root /

RUN find $(find / -name "libtorch_cpu.so" | sed -e s/libtorch_cpu.so/*/) -exec cp {} /lib/ \;

EXPOSE 8000
CMD ["branswers"]
