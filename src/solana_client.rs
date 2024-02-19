use anyhow::{anyhow, Context};
use reqwest::{Client, Response};
use std::time::Duration;

use crate::domain::DataSend;

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
        let data = DataSend::new();
        self.send_request(data)
            .await
            .context("Failed to connect to remote node")?;
        tracing::info!("Handshake ended succesfully!");
        Ok(())
    }

    #[tracing::instrument(name = "Invoking get version", skip(self))]
    pub async fn get_version(&self) -> Result<Response, anyhow::Error> {
        // TODO Check Http Response - Based on this, either continue if OK or return Enum ConnectionError
        // TODO Check Data returned - Potentially malicious, parse in Domain struct. If Ok, return
        // Ok(), if not return DataError
        let data = DataSend::new();
        match self.send_request(data).await {
            Ok(response) => Ok(response),
            Err(_) => Err(anyhow!("Error")),
        }
    }

    #[tracing::instrument(name = "Sending HTTP request", skip(self))]
    async fn send_request(&self, data: DataSend) -> Result<Response, reqwest::Error> {
        self.http_client.post(&self.uri).json(&data).send().await
    }
}
