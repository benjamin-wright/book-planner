deps-osx:
    rustup target add wasm32-wasi
    brew install kind
    brew install knative/client/kn
    brew install knative-sandbox/kn-plugins/quickstart
    helm repo add kwasm http://kwasm.sh/kwasm-operator/

init:
    mkdir -p .scratch
    kn quickstart kind --install-serving
    kind export kubeconfig -n knative --kubeconfig .scratch/kubeconfig
    kubectl patch configmap config-deployment -n knative-serving -p '{"data": {"registries-skipping-tag-resolving": "localhost:5001"} }'
    helm upgrade --install -n kwasm --create-namespace kwasm-operator kwasm/kwasm-operator
    kubectl annotate node --all --overwrite kwasm.sh/kwasm-node=true
    helm upgrade --install infra deploy/infra
    kubectl patch configmap config-features -n knative-serving -p '{"data": {"kubernetes.podspec-runtimeclassname": "enabled"} }'

clean:
    kind delete cluster -n knative
