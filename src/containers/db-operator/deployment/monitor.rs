use tokio::sync::mpsc::{channel, Receiver};

use k8s_openapi::api::apps::v1::Deployment;
use futures::{TryStreamExt};
use kube::{
    Api,
    Client,
    runtime::{
        watcher,
        watcher::Event
    },
    api::{ListParams},
};

#[derive(Clone)]
pub struct Monitor {
    client: Client,
    db_state: ReadyState
}

#[derive(Clone, Debug)]
pub enum ReadyState {
    Missing,
    NotReady,
    Ready
}

impl Monitor {
    pub async fn new() -> Result<Monitor,Box<dyn std::error::Error + Send + Sync>> {
        let client = match Client::try_default().await {
            Ok(client) => client,
            Err(err) => return Err(err.into())
        };

        Ok(Monitor{client: client, db_state: ReadyState::Missing})
    }

    pub fn start(&self) -> Receiver<()> {
        let _depl_state_updates = self.watch_cockroach_deployments();
        let (_tx, rx) = channel(1);

        return rx;
    }


    fn watch_cockroach_deployments(&self) -> Receiver<ReadyState> {
        let (tx, rx) = channel(1);
        let client = self.client.clone();

        tokio::spawn(async move {
            let pods: Api<Deployment> = Api::default_namespaced(client.clone());
            let lp = ListParams::default().fields("metadata.name=cockroach-db");
            
            let result = watcher(pods, lp).try_for_each(|event| async {
                let result = match event {
                    Event::Applied(deployment) => tx.send(Monitor::get_deployment_ready(deployment)).await,
                    Event::Deleted(_deployment) => tx.send(ReadyState::Missing).await,
                    Event::Restarted(deployments) => {
                        if deployments.len() == 1 {
                            let deployment = deployments[0].clone();
                            tx.send(Monitor::get_deployment_ready(deployment)).await
                        } else {
                            tx.send(ReadyState::Missing).await
                        }
                    }
                };

                match result {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        println!("Error sending response: {:?}", err);
                        Ok(())
                    }
                }
            }).await;

            match result {
                Ok(_) => {},
                Err(err) => println!("Error in watcher: {:?}", err)
            }
        });

        return rx;
    }

    fn get_deployment_ready(d: Deployment) -> ReadyState {
        let status = match d.status {
            Some(status) => status,
            None => return ReadyState::NotReady
        };

        let replicas = match status.available_replicas {
            Some(replicas) => replicas,
            None => return ReadyState::NotReady
        };

        if replicas > 0 {
            ReadyState::Ready
        } else {
            ReadyState::NotReady
        }
    }

    pub async fn cockroach_exists(&self) -> Result<bool,Box<dyn std::error::Error + Send + Sync>> {
        let pods: Api<Deployment> = Api::default_namespaced(self.client.clone());

        match pods.get("cockroach-db").await {
            Ok(_) => Ok(true),
            Err(kube::Error::Api(ae)) => {
                match ae.code {
                    404 => Ok(false),
                    _ => Err(ae.into())
                }
            }
            Err(err) => Err(err.into())
        }
    }
}

