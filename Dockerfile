FROM rust:buster

COPY . /app

RUN cargo install --root /opt/taskmaster --path /app
COPY configs /opt/taskmaster/configs

ENTRYPOINT [ "/bin/bash" ]
