use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize)]
pub struct DataSend {
    jsonrpc: String,
    id: u8,
    method: String,
}

impl DataSend {
    pub fn new(method: String) -> Self {
        let jsonrpc = "2.0".to_string();
        let id = 1;
        Self {
            jsonrpc,
            id,
            method,
        }
    }
}

impl Default for DataSend {
    fn default() -> Self {
        Self::new("getVersion".to_string())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataReceive {
    jsonrpc: String,
    result: DataReceiveResult,
    id: u8,
}

impl Default for DataReceive {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: DataReceiveResult::default(),
            id: 1,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataReceiveResult {
    #[serde(rename = "feature-set")]
    feature_set: u32,
    #[serde(rename = "solana-core")]
    solana_core: String,
}

impl Default for DataReceiveResult {
    fn default() -> Self {
        Self {
            feature_set: 3580551090,
            solana_core: "1.17.23".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataReceiveError {
    jsonrpc: String,
    error: DataReceiveResultError,
    id: u8,
}

impl Default for DataReceiveError {
    fn default() -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            error: DataReceiveResultError::default(),
            id: 1,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DataReceiveResultError {
    code: i32,
    message: String,
}

impl Default for DataReceiveResultError {
    fn default() -> Self {
        Self {
            code: -32601,
            message: "Method not found".to_string(),
        }
    }
}
