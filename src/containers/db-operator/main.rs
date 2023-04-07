use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{ListParams},
    Api,
    Client
};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);
    let lp = ListParams::default();
    let p = pods.list(&lp).await?;
    println!("Got pod with containers: {:?}", p);

    Ok(())
}
