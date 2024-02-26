use anyhow::Context;
use reqwest::{Client, Response, StatusCode};
use std::time::Duration;

use crate::solana::domain::{DataReceive, DataReceiveError, DataSend};

/// Client that is used to establish communication with the remote node.
#[derive(Debug)]
pub struct SolanaClient {
    http_client: Client,
    uri: String,
}

/// Error enumeration to represent higher abstraction level of errors.
#[derive(thiserror::Error, Debug)]
pub enum SolanaClientError {
    #[error("HTTP Response error: Remote node did not return 200 OK")]
    HttpResponseError,
    #[error("Sent data error: Sent data to remote node is not valid")]
    SentDataError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl SolanaClient {
    /// Creates a Solana client based on provided uri and timeout arguments
    ///
    /// The uri is an HTTP URL, usually for port 8899, as in
    /// http://127.0.0.1:8899. To access official Solana devnet,
    /// <http://api.devnet.solana.com> without port has to be used.
    ///
    /// # Example
    ///
    /// ```
    /// use p2p_handshake_solana::solana::client::SolanaClient;
    ///
    /// let uri = "http://127.0.0.1:8899".to_string();
    /// let timeout = 200; // In miliseconds
    /// let solana_client = SolanaClient::new(uri, timeout);
    /// ```
    pub fn new(uri: String, timeout: u64) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_millis(timeout))
            .build()
            .unwrap();
        Self { http_client, uri }
    }

    /// Solana client performs handshake with the remote node provided in
    /// [`new`]. Handshake is a wrapper around the [`get_version`] function
    /// and it returns if it succeeded or not.
    ///
    /// [`new`]: SolanaClient::new
    /// [`get_version`]: SolanaClient::get_version
    ///
    /// # Example
    ///
    /// ```
    /// use p2p_handshake_solana::solana::client::SolanaClient;
    ///
    /// let uri = "http://api.devnet.solana.com".to_string();
    /// let timeout = 200; // In miliseconds
    /// let solana_client = SolanaClient::new(uri, timeout);
    /// let result = async {
    ///     solana_client.handshake().await
    /// };
    /// ```
    pub async fn handshake(&self) -> Result<(), anyhow::Error> {
        let data = DataSend::default();
        self.get_version(data)
            .await
            .context("Failed to invoke get version")?;
        Ok(())
    }

    /// Solana client calls the getVersion method specified in here
    /// <https://solana.com/docs/rpc/http/getversion>. It checks the HTTP response,
    /// returned data and eventually verifies if the returned data is correct.
    /// It is necessary to provide data which is going to be sent to the remote node.
    /// Template data has been created in [`DataSend`].
    ///
    /// [`DataSend`]: DataSend
    /// # Example
    ///
    /// ```
    /// use p2p_handshake_solana::solana::client::SolanaClient;
    /// use p2p_handshake_solana::solana::domain::DataSend;
    ///
    /// let uri = "http://api.devnet.solana.com".to_string();
    /// let timeout = 200; // In miliseconds
    /// let solana_client = SolanaClient::new(uri, timeout);
    /// let data = DataSend::default();
    /// let response = async {
    ///     solana_client.get_version(data).await.unwrap()
    /// }; // If no error, proper data
    /// ```
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
        let data_receive = self.verify_returned_data(data)?;
        Ok(data_receive)
    }

    /// Http client sends request to the remote node
    async fn send_request(&self, data: DataSend) -> Result<Response, reqwest::Error> {
        self.http_client.post(&self.uri).json(&data).send().await
    }

    /// Returned data verification. First it tries to deserialize it in DataReceive. If it is
    /// OK, then verify if the data is as expected. If it is, verification has succeeded,
    /// otherwise unexpected data is received. If the initial deserialize fails, it tries
    /// to serialize it in DataReceiveError, struct that is according to the returned error
    /// from remote node. If it succeeds, proper error has been returned. Otherwise, unexpected
    /// error form has been received from the remote node.
    fn verify_returned_data(&self, data: String) -> Result<DataReceive, SolanaClientError> {
        match serde_json::from_str::<DataReceive>(&data) {
            Ok(data_json) => {
                if !data_json.verify() {
                    return Err(SolanaClientError::UnexpectedError(anyhow::anyhow!(
                        "Unexpected data returned from remote node, possibly corrupted"
                    )));
                }
                Ok(data_json)
            }
            Err(_) => match serde_json::from_str::<DataReceiveError>(&data) {
                Ok(_) => Err(SolanaClientError::SentDataError),
                Err(e) => Err(SolanaClientError::UnexpectedError(anyhow::anyhow!(
                    "Unexpected error response provided from the node: {}",
                    e
                ))),
            },
        }
    }
}
