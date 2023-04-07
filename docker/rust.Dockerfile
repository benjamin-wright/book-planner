FROM debian:stable-slim

WORKDIR /src

COPY app /src/app

ENTRYPOINT [ "/src/app" ]