FROM rust:slim

COPY . /app

RUN cargo install --root /usr --path /app

ENTRYPOINT [ "/bin/bash" ]
