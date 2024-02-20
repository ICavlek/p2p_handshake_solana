use anyhow::Context;
use reqwest::{Client, Response, StatusCode};
use std::time::Duration;

use crate::domain::{DataReceive, DataSend};

#[derive(Debug)]
pub struct SolanaClient {
    http_client: Client,
    uri: String,
}

#[derive(thiserror::Error, Debug)]
pub enum SolanaClientError {
    #[error("HTTP Response error, remote node did not return 200 OK")]
    HttpResponseError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
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
        // TODO Check Data returned - Potentially malicious, parse in Domain struct. If Ok, return
        // Ok(), if not return DataError
        let data = DataSend::new();
        self.get_version(data)
            .await
            .context("Failed to invoke get version")?;
        tracing::info!("Successfully performed handshake");
        Ok(())
    }

    #[tracing::instrument(name = "Invoking get version", skip(self))]
    pub async fn get_version(&self, data: DataSend) -> Result<DataReceive, SolanaClientError> {
        let response = self
            .send_request(data)
            .await
            .context("Failed to connect to remote node")?;
        match response.status() {
            StatusCode::OK => tracing::info!("Remote node returned 200 OK"),
            _ => return Err(SolanaClientError::HttpResponseError),
        };

        let data = response.text().await.map_err(|e| {
            anyhow::anyhow!(
                "Something went wrong with getting data, original error: {}",
                e
            )
        })?;
        let data_receive = serde_json::from_str::<DataReceive>(&data).unwrap();
        Ok(data_receive)
    }

    #[tracing::instrument(name = "Sending HTTP request", skip(self))]
    async fn send_request(&self, data: DataSend) -> Result<Response, reqwest::Error> {
        self.http_client.post(&self.uri).json(&data).send().await
    }
}
