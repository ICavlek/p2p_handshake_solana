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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DataReceive {
    jsonrpc: String,
    result: DataReceiveResult,
    id: u8,
}

impl Default for DataReceive {
    fn default() -> Self {
        Self::new("2.0".to_string())
    }
}

impl DataReceive {
    pub fn new(jsonrpc: String) -> Self {
        Self {
            jsonrpc,
            result: DataReceiveResult::default(),
            id: 1,
        }
    }

    pub fn verify(&self) -> bool {
        let default_receive = DataReceive::default();
        self.eq(&default_receive)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::DataReceive;
    use super::DataReceiveError;

    #[test]
    fn deserialize_string_to_data_receive_error() {
        let error_response =
            r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":1}"#;
        let data = serde_json::from_str::<DataReceiveError>(error_response);
        assert!(data.is_ok());
    }

    #[test]
    fn deserialize_wrong_string_to_data_receive_error() {
        let error_response = r#"{error":{"code":-32601,"message":"Method not found"},"id":1}"#;
        let data = serde_json::from_str::<DataReceiveError>(error_response);
        assert!(data.is_err());
    }

    #[test]
    fn deserialize_wrong_string_to_data_receive() {
        let error_response =
            r#"{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":1}"#;
        let data = serde_json::from_str::<DataReceive>(error_response);
        assert!(data.is_err());
    }
}
