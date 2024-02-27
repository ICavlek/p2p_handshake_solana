use std::collections::HashMap;

use tokio::task::JoinHandle;

use super::client::SolanaClient;

/// Module to handle multiple solana client handshakes
pub struct SolanaClientPool {
    tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>>,
}

impl SolanaClientPool {
    /// Creates mutltiple solana clients from the vector of uri's.
    ///
    /// #Example
    ///
    /// ```
    /// use p2p_handshake_solana::solana::client_pool::SolanaClientPool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let clients = vec![
    ///         "http://127.0.0.1:8899".to_string(),
    ///         "http://api.testnet.solana.com".to_string(),
    ///         "http://api.devnet.solana.com".to_string()
    ///     ];
    ///     let timeout = 500; // miliseconds
    ///     let client_pool = SolanaClientPool::new(clients, timeout);
    /// }
    /// ```
    pub fn new(nodes: Vec<String>, timeout: u64) -> SolanaClientPool {
        let mut tasks: HashMap<String, JoinHandle<Result<(), anyhow::Error>>> = HashMap::new();
        for node in nodes {
            let task =
                tokio::task::spawn(SolanaClientPool::perform_handshake(node.clone(), timeout));
            tasks.insert(node, task);
        }
        Self { tasks }
    }
    /// Runs mutltiple solana clients from the SolanaClientPool.
    ///
    /// #Example
    ///
    /// ```
    /// use p2p_handshake_solana::solana::client_pool::SolanaClientPool;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let clients = vec![
    ///         "http://127.0.0.1:8899".to_string(),
    ///         "http://api.testnet.solana.com".to_string(),
    ///         "http://api.devnet.solana.com".to_string()
    ///     ];
    ///     let timeout = 500; // miliseconds
    ///     let client_pool = SolanaClientPool::new(clients, timeout);
    ///     client_pool.run().await.unwrap();
    /// }
    /// ```
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
            if let Ok(()) = result {
                tracing::info!("Successfully performed handshake for Node {}", node);
            }
        }
        Ok(())
    }

    /// Runs handshake on all provided nodes.
    #[tracing::instrument("Performing handshake", skip(timeout))]
    async fn perform_handshake(uri: String, timeout: u64) -> Result<(), anyhow::Error> {
        let solana_client = SolanaClient::new(uri, timeout);
        match solana_client.handshake().await {
            Ok(_) => Ok(()),
            Err(e) => {
                tracing::error!(error.cause_chain = ?e, error.message = %e, "Error performing handshake: {}", e);
                Err(anyhow::anyhow!("Error performing handshake: {}", e))
            }
        }
    }
}
