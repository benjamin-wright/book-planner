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
                    Event::Applied(deployment) => {
                        let replicas = Monitor::get_deployment_replicas(deployment);

                        println!("cockroachDB update: {:?} ready", replicas);

                        if replicas > 0 {
                            tx.send(ReadyState::Ready).await
                        } else {
                            tx.send(ReadyState::NotReady).await
                        }
                    },
                    Event::Deleted(_deployment) => {
                        println!("cockroachDB deleted");
                        tx.send(ReadyState::Missing).await
                    },
                    Event::Restarted(deployments) => {
                        println!("cockroachDB watch restarted");
                        if deployments.len() == 1 {
                            let deployment = deployments[0].clone();
                            let replicas = Monitor::get_deployment_replicas(deployment);

                            if replicas > 0 {
                                println!("cockroachdb deployment ready ({} replicas)", replicas);
                                tx.send(ReadyState::Ready).await
                            } else {
                                println!("cockroachdb deployment not ready");
                                tx.send(ReadyState::NotReady).await
                            }
                        } else {
                            println!("No cockroachdb deployment found");
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

    fn get_deployment_replicas(d: Deployment) -> i32 {
        let status = match d.status {
            Some(status) => status,
            None => return 0
        };

        match status.available_replicas {
            Some(replicas) => replicas,
            None => return 0
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

