FROM rust:alpine3.19

LABEL maintainer="JaimeGM96"

WORKDIR /app/test

COPY Cargo.toml .

RUN adduser -D jaime \
    && chown -R jaime:jaime /app/test
USER jaime

RUN mkdir src \
    && echo "// dummy file" > src/ruta.rs \
    && cargo build

ENTRYPOINT [ "cargo", "test" ]
