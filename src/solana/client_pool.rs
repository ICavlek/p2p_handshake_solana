use std::collections::HashMap;

use tokio::task::JoinHandle;

use super::client::SolanaClient;

pub struct SolanaClientPool {
    tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>>,
}

impl SolanaClientPool {
    pub fn new(nodes: Vec<String>, timeout: u64) -> SolanaClientPool {
        let mut tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>> = HashMap::new();
        for node in nodes {
            let task =
                tokio::task::spawn(SolanaClientPool::perform_handshake(node.clone(), timeout));
            tasks.insert(node, task);
        }
        Self { tasks }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        for (node, task) in self.tasks.into_iter() {
            let result = match task.await {
                Ok(result) => result,
                Err(e) => {
                    tracing::error!(
                        error.cause_chain = ?e,
                        error.message = %e,
                    );
                    Err(anyhow::anyhow!(e))
                }
            };
            match result {
                Ok(()) => {
                    tracing::info!("Successfully performed handshake for Node {}", node);
                }
                Err(e) => {
                    tracing::error!(error.cause_chain = ?e, error.message = %e,"Error with Node {}", node);
                }
            }
        }
        Ok(())
    }

    #[tracing::instrument("Performing handshake", skip(timeout))]
    async fn perform_handshake(uri: String, timeout: u64) -> Result<(), anyhow::Error> {
        let solana_client = SolanaClient::new(uri, timeout);
        match solana_client.handshake().await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!("Error performing handshake: {}", e);
                Err(anyhow::anyhow!("Error performing handshake: {}", e))
            }
        }
    }
}
