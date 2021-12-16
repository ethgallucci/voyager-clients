use std::error::Error;
use serde_json::{Value as JsonValue};

use super::to_pretty::to_string_pretty;

pub fn neo() -> Result<String, Box<dyn Error>> {
    let res: String = ureq::get("https://api.nasa.gov/neo/rest/v1/feed?start_date=2021-12-11&end_date=2021-12-15&api_key=qxUB72hYa6OfctXigKLdRhATtGM9iCOKCKuBhQ19").call()?.into_string()?;

    let neo = to_string_pretty(res).unwrap();

    Ok(neo)
}
