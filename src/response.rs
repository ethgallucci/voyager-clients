use serde_json::Value as JsonValue;
use serde_derive::{Serialize, Deserialize};
use std::error::Error;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response {
    json: JsonValue,
}

impl Response {
    pub fn new(json: JsonValue) -> Self {
        Response { json }
    }

    pub fn bytedump(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let bytes: Vec<u8> = serde_json::to_vec(&self.json)?;
        Ok(bytes)
    }
}

/// Handles responses
pub fn into_response(res: &str) -> Result<Response, Box<dyn Error>> {
    let json: JsonValue = serde_json::from_str(res)?;

    Ok(Response::new(json))
}