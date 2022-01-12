
use std::error::Error;

use crate::key;
use crate::pretty::*;

/// Base Client for ApodClient api
#[derive(Debug, PartialEq)]
pub struct ApodClient {
    base_url: String,
    date: Option<String>,
}

impl ApodClient {
    /// Constructor
    pub fn new() -> Self {
        ApodClient {
            base_url: String::from("https://api.nasa.gov/planetary/apod?"),
            date: None,
        }
    }

    /// Set the date to query
    pub fn set_date(&mut self, date: String) {
        self.date = Some(date);
    }

    /// Query function
    pub fn query(&self) -> Result<String, Box<dyn Error>> {
        let key: String = key::from_dotenv()?;

        if self.date.is_none() {
            let url = format!("{}api_key={}", self.base_url, key);

            println!("Url: {}", url);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let apod = to_string_pretty(res).unwrap();

            Ok(apod)
        } else {
            let date = self.date.as_ref().unwrap();
            let url = format!("{}date={}&api_key={}", self.base_url, date, key);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let apod = to_string_pretty(res).unwrap();

            Ok(apod)
        }
    }
}