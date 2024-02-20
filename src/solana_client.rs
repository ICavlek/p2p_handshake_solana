use anyhow::Context;
use reqwest::{Client, Response, StatusCode};
use std::time::Duration;

use crate::domain::{DataReceive, DataReceiveError, DataSend};

#[derive(Debug)]
pub struct SolanaClient {
    http_client: Client,
    uri: String,
}

#[derive(thiserror::Error, Debug)]
pub enum SolanaClientError {
    #[error("HTTP Response error: Remote node did not return 200 OK")]
    HttpResponseError,
    #[error("Sent data error: Sent data is not valid")]
    SentDataError,
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
        let data = DataSend::default();
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
            .context("Failed to get response from the remote node")?;
        match response.status() {
            StatusCode::OK => tracing::info!("Remote node returned 200 OK"),
            _ => return Err(SolanaClientError::HttpResponseError),
        };
        let data = response
            .text()
            .await
            .context("Something went wrong with extracting data")?;
        let data_receive = match serde_json::from_str::<DataReceive>(&data) {
            Ok(data_json) => data_json,
            Err(_) => match serde_json::from_str::<DataReceiveError>(&data) {
                Ok(_) => return Err(SolanaClientError::SentDataError),
                Err(e) => {
                    return Err(SolanaClientError::UnexpectedError(anyhow::anyhow!(
                        "Unexpected error response provided from the node: {}",
                        e
                    )))
                }
            },
        };
        Ok(data_receive)
    }

    #[tracing::instrument(name = "Sending HTTP request", skip(self))]
    async fn send_request(&self, data: DataSend) -> Result<Response, reqwest::Error> {
        self.http_client.post(&self.uri).json(&data).send().await
    }
}
