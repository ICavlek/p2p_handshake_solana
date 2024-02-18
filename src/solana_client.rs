use anyhow::{anyhow, Context};
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
    pub async fn handshake(&self) -> Result<(), anyhow::Error> {
        self.send_request()
            .await
            .context("Failed to connect to remote node")?;
        tracing::info!("Handshake ended succesfully!");
        Ok(())
    }

    #[tracing::instrument(name = "Invoking get health", skip(self))]
    pub async fn get_health(&self) -> Result<Response, anyhow::Error> {
        match self.send_request().await {
            Ok(response) => Ok(response),
            Err(_) => Err(anyhow!("Error")),
        }
    }

    #[tracing::instrument(name = "Sending HTTP request", skip(self))]
    async fn send_request(&self) -> Result<Response, reqwest::Error> {
        self.http_client
            .post(&self.uri)
            .header("Content-Type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"method":"getHealth"}"#)
            .send()
            .await
    }
}
