use anyhow::Context;
use reqwest::{Client, Response};
use std::time::Duration;

#[derive(Debug)]
pub struct SolanaClient {
    http_client: Client,
    uri: String,
}

impl SolanaClient {
    #[tracing::instrument(name = "Init Client")]
    pub fn new(uri: String) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_millis(200))
            .build()
            .unwrap();
        Self { http_client, uri }
    }

    #[tracing::instrument(name = "Handshake", skip(self))]
    pub async fn handshake(&self) -> Result<Response, anyhow::Error> {
        let response = self
            .http_client
            .post(&self.uri)
            .header("Content-Type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"method":"getHealth"}"#)
            .send()
            .await
            .context("Failed to connect to remote node")?;
        Ok(response)
    }
}
