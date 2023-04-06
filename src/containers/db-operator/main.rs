use k8s_openapi::api::core::v1::Pod;

#[tokio::main()]
async fn main() {
    let pods: Api<Pod> = Api::default_namespaced(client);
    let p = pods.get("blog").await?;
    println!("Got blog pod with containers: {:?}", p.spec.unwrap().containers);
}