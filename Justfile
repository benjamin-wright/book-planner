deps-osx:
    rustup target add wasm32-wasi
    brew install kind
    brew install knative/client/kn
    brew install knative-sandbox/kn-plugins/quickstart
    helm repo add kwasm http://kwasm.sh/kwasm-operator/

init: infra-cluster infra-resources infra-wasm

@infra-cluster:
    kn quickstart kind --install-serving
    kubectl patch configmap config-deployment -n knative-serving -p '{"data": {"registries-skipping-tag-resolving": "localhost:5001"} }'

@infra-resources:
    kubectl patch deployment activator -n knative-serving -p '{"spec":{"template":{"spec":{"containers":[{"name":"activator","resources":{"requests":{"cpu": "100m"}}}]}}}}'
    kubectl patch deployment net-kourier-controller -n knative-serving -p '{"spec":{"template":{"spec":{"containers":[{"name":"controller","resources":{"requests":{"cpu": "100m"}}}]}}}}'
    kubectl patch deployment 3scale-kourier-gateway -n kourier-system -p '{"spec":{"template":{"spec":{"containers":[{"name":"kourier-gateway","resources":{"requests":{"cpu": "100m"}}}]}}}}'

@infra-wasm:
    helm upgrade --install -n kwasm --create-namespace kwasm-operator kwasm/kwasm-operator
    kubectl annotate node --all --overwrite kwasm.sh/kwasm-node=true
    helm upgrade --install infra deploy/infra
    kubectl patch configmap config-features -n knative-serving -p '{"data": {"kubernetes.podspec-runtimeclassname": "enabled"} }'

clean:
    -kind delete cluster -n knative
    -docker stop kind-registry
    -docker rm kind-registry

tools:
    cd src/tools && cargo build
    mkdir -p bin
    cp src/tools/target/debug/copy-wasms bin/copy-wasms

wasm:
    cd src/wasm && cargo build --target wasm32-wasi
    rm -rf src/wasm/bin
    copy-wasms src/wasm target/wasm32-wasi/debug

containers:
    cd src/containers && cargo zigbuild --target x86_64-unknown-linux-gnu --release
    rm -rf src/containers/bin
    copy-wasms src/containers target/x86_64-unknown-linux-gnu/release

build: tools wasm containers

app APP_NAME:
    cargo build --target wasm32-wasi --bin {{APP_NAME}}

    mkdir -p bin/{{APP_NAME}}
    cp "target/wasm32-wasi/debug/{{APP_NAME}}.wasm" "bin/{{APP_NAME}}/app.wasm"

fn_image APP_NAME IMAGE_TAG:
    docker buildx build \
        --platform wasi/wasm32 \
        -f docker/wasm.Dockerfile \
        -t {{IMAGE_TAG}} \
        "src/wasm/bin/{{APP_NAME}}"

container_image APP_NAME IMAGE_TAG:
    docker buildx build \
        -f docker/rust.Dockerfile \
        -t {{IMAGE_TAG}} \
        "src/containers/bin/{{APP_NAME}}"

endpoint APP_NAME:
    curl http://{{APP_NAME}}.default.127.0.0.1.sslip.io

dev:
    tilt up