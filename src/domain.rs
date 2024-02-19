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
