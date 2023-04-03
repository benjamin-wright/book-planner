FROM scratch

ARG EXE_NAME

COPY ${EXE_NAME} /app.wasm

ENTRYPOINT [ "/app.wasm" ]