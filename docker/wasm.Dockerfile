FROM scratch

COPY app.wasm /app.wasm

ENTRYPOINT [ "/app.wasm" ]