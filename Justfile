deps-osx:
    rustup target add wasm32-wasi
    brew install kind
    brew install knative/client/kn
    brew install knative-sandbox/kn-plugins/quickstart
    helm repo add kwasm http://kwasm.sh/kwasm-operator/

init: cluster resources wasm

cluster:
    kn quickstart kind --install-serving
    kubectl patch configmap config-deployment -n knative-serving -p '{"data": {"registries-skipping-tag-resolving": "localhost:5001"} }'
    '{"spec":{"template":{"spec":{"containers":[{"name":"controller",env:{"KOURIER_EXTAUTHZ_HOST":""}}]}}}}'

resources:
    kubectl patch deployment activator -n knative-serving -p '{"spec":{"template":{"spec":{"containers":[{"name":"activator","resources":{"requests":{"cpu": "100m"}}}]}}}}'
    kubectl patch deployment net-kourier-controller -n knative-serving -p '{"spec":{"template":{"spec":{"containers":[{"name":"controller","resources":{"requests":{"cpu": "100m"}}}]}}}}'
    kubectl patch deployment 3scale-kourier-gateway -n kourier-system -p '{"spec":{"template":{"spec":{"containers":[{"name":"kourier-gateway","resources":{"requests":{"cpu": "100m"}}}]}}}}'

wasm:
    helm upgrade --install -n kwasm --create-namespace kwasm-operator kwasm/kwasm-operator
    kubectl annotate node --all --overwrite kwasm.sh/kwasm-node=true
    helm upgrade --install infra deploy/infra
    kubectl patch configmap config-features -n knative-serving -p '{"data": {"kubernetes.podspec-runtimeclassname": "enabled"} }'

clean:
    kind delete cluster -n knative
