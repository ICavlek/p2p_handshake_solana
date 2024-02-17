use reqwest::Client;
use std::time::Duration;

pub struct SolanaClient {
    http_client: Client,
}

impl SolanaClient {
    pub fn new() -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_millis(200))
            .build()
            .unwrap();
        Self { http_client }
    }

    pub async fn handshake(&self) -> String {
        self.http_client
            .post("http://127.0.0.1:8899")
            .header("Content-Type", "application/json")
            .body(r#"{"jsonrpc":"2.0","id":1,"method":"getHealth"}"#)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }
}
