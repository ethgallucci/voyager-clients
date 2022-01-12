use serde_json::Value as JsonValue;
use std::error::Error;

/// Converts a JSON blob into a pretty string.
pub fn to_string_pretty(res: String) -> Result<String, Box<dyn Error>> {
    let json: JsonValue = serde_json::from_str(&res).unwrap();
    let pretty: String = serde_json::to_string_pretty(&json).unwrap();

    Ok(pretty)
}
