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
    #[allow(dead_code)]
    jsonrpc: String,
    #[allow(dead_code)]
    result: DataReceiveResult,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    feature_set: u32,
    #[serde(rename = "solana-core")]
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    jsonrpc: String,
    #[allow(dead_code)]
    error: DataReceiveResultError,
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    code: i32,
    #[allow(dead_code)]
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
