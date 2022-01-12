use std::error::Error;

use crate::key;
use crate::pretty::*;

/// Base Client for Solar Flare API
#[derive(Debug, PartialEq)]
pub struct SolarFlare {
    base_url: String,
}

impl SolarFlare {
    /// New Base Client
    pub fn new() -> Self {
        SolarFlare {
            base_url: String::from("https://api.nasa.gov/DONKI/FLR?startDate="),
        }
    }

    /// Query method
    pub fn query(&self, start: String, end: String) -> Result<String, Box<dyn Error>> {
        let key: String = key::from_dotenv()?;

        let url: String = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
        println!("Starting solar query from {}, to {}.", start, end);

        let res: String = ureq::get(&url).call()?.into_string()?;
        let solar = to_string_pretty(res).unwrap();

        Ok(solar)
    }
}

/// Base Client for GeoMagnetic Storms API
#[derive(Debug, PartialEq)]
pub struct GeoMagnetic {
    base_url: String,
}

impl GeoMagnetic {
    /// Create new Base Client
    pub fn new() -> Self {
        GeoMagnetic {
            base_url: String::from("https://api.nasa.gov/DONKI/GST?startDate="),
        }
    }

    /// Query method
    pub fn query(&self, start: String, end: String) -> Result<String, Box<dyn Error>> {
        let key: String = key::from_dotenv()?;

        let url: String = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
        println!("Starting GeoMagnetic query from {}, to {}.", start, end);

        let res: String = ureq::get(&url).call()?.into_string()?;
        let mag = to_string_pretty(res).unwrap();

        Ok(mag)
    }
}

/// For interacting with the Coronal Mass Ejction API.
///
/// # Example
/// ```
/// use voyager_client::donki;
///
/// // Instantiate Base Client
/// let base = donki::CoronalMassEjection::new();
///
/// /// // Setup Timings
/// let start = String::from("2022-01-01");
/// let end = String::from("2022-01-07");
///
/// // Query Endpoint
/// let res = base.query(start, end).unwrap();
/// ```

#[derive(Debug, PartialEq)]
pub struct CoronalMassEjection {
    base_url: String,
}

impl CoronalMassEjection {
    /// Create a new Base Client
    pub fn new() -> Self {
        CoronalMassEjection {
            base_url: String::from("https://api.nasa.gov/DONKI/CME?startDate="),
        }
    }
    /// Query method
    pub fn query(&self, start: String, end: String) -> Result<String, Box<dyn Error>> {
        let key = key::from_dotenv()?;

        let url = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
        println!("Starting CME query from {}, to {}.", start, end);

        let res: String = ureq::get(&url).call()?.into_string()?;
        let cme = to_string_pretty(res).unwrap();

        Ok(cme)
    }
}

/// SEP base client
#[derive(Debug, PartialEq)]
pub struct SolarEnergeticParticle {
    base_url: String,
}

impl SolarEnergeticParticle {
    /// Create a new SEP base client
    pub fn new() -> Self {
        SolarEnergeticParticle {
            base_url: String::from("https://api.nasa.gov/DONKI/SEP?")
        }
    }
    /// Query method
    pub fn query(&self, start: String, end: String) -> Result<String, Box<dyn Error>> {
        let key = key::from_dotenv()?;

        let url = format!(
            "{}startDate={}&endDate={}&api_key={}",
            self.base_url, start, end, key
        );

        let res = ureq::get(&url).call()?.into_string()?;
        let sep = to_string_pretty(res).unwrap();

        Ok(sep)
    }
}