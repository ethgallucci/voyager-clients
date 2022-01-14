use std::error::Error;

use crate::key;
use crate::response::*;

#[derive(Debug)]
pub struct Neo {
    base_url: String,
}

impl Neo {
    pub fn new() -> Self {
        Neo {
            base_url: String::from("https://api.nasa.gov/neo/rest/v1/feed?start_date="),
        }
    }

    pub fn query(&self, start: String, end: String) -> Result<Response, Box<dyn Error>> {
        let key: String = key::from_dotenv()?;

        let url: String = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
        println!("Starting Neo query from {}, to {}.", start, end);

        let res: String = ureq::get(&url).call()?.into_string()?;
        let neo = into_response(res.as_str()).unwrap();

        Ok(neo)
    }
}
