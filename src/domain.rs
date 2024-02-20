#[derive(Debug, serde::Serialize)]
pub struct DataSend {
    jsonrpc: String,
    id: u8,
    method: String,
}

impl DataSend {
    pub fn new() -> Self {
        let jsonrpc = "2.0".to_string();
        let id = 1;
        let method = "getVersion".to_string();
        Self {
            jsonrpc,
            id,
            method,
        }
    }
}

impl Default for DataSend {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct DataReceive {
    #[allow(dead_code)]
    jsonrpc: String,
    #[allow(dead_code)]
    result: DataReceiveResult,
    #[allow(dead_code)]
    id: u8,
}

#[derive(Debug, serde::Deserialize)]
pub struct DataReceiveResult {
    #[serde(rename = "feature-set")]
    #[allow(dead_code)]
    feature_set: u32,
    #[serde(rename = "solana-core")]
    #[allow(dead_code)]
    solana_core: String,
}
