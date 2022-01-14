use std::error::Error;

use crate::key;
use crate::response::*;

#[derive(Debug, PartialEq)]
pub struct InsightWeather {
    base_url: String,
}

impl InsightWeather {
    pub fn new() -> Self {
        InsightWeather {
            base_url: String::from("https://api.nasa.gov/insight_weather/?api_key="),
        }
    }

    pub fn query(&self) -> Result<Response, Box<dyn Error>> {
        let key = key::from_dotenv()?;

        let url = format!("{}{}&feedtype=json&ver=1.0", self.base_url, key);
        println!("Starting Inisght Weather query: {}", url);

        let res: String = ureq::get(&url).call()?.into_string()?;
        let mrover = into_response(res.as_str()).unwrap();

        Ok(mrover)
    }
}
