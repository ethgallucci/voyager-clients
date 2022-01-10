//!
//! # Sample program with voyager_client
//! ```
//! use voyager_client::{donki_client, timing, keys};
//!
//! // Run this function only once - Or, cargo install this crate to install the CLI binaries, then run voyager_client set key [YOUR_KEY]
//! // keys::set_key("DEMO_KEY");
//!
//! // Instantiate a Base Client
//! let base_donki_client = donki_client::Solar::new();
//!
//! // Setup timing parameters
//! let start = String::from("2018-01-01");
//! let end = timing::today();
//!
//! // Query the API
//! let res = base_donki_client.query(start, end).unwrap();
//! ```
//! This will fetch a response from the magnetic storms endpoint, and convert it
//! into a prettyfied String in JSON format

#![allow(dead_code)]

/// Handling API keys for NASA's open APIs.
/// Includes methods for storing, and retrieving keys for any user
/// 
/// # Retrieving a key
/// ```
/// use voyager_client::keys::*;
/// let key = get_key().unwrap();
/// ```
pub mod keys {
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;

    use users::{get_current_uid, get_user_by_uid};

    /// Stores a key.
    pub fn set_key(key: &str) -> std::io::Result<()> {
        let username = get_user();

        let path_to_voyager = format!("/Users/{}/voyager", username);
        std::fs::create_dir(&path_to_voyager).unwrap();

        let path_to_key = format!("{}/.api_key.txt", &path_to_voyager);
        let mut file = File::create(path_to_key)?;
        file.write_all(&key.as_bytes())?;
        println!("Set key for {}", username);
        Ok(())
    }

    /// Retrieves a key
    pub fn get_key() -> Result<String, Box<dyn Error>> {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        let path_to_key = format!(
            "/Users/{}/voyager/.api_key.txt",
            user.name().to_string_lossy()
        );
        let key = fs::read_to_string(&path_to_key).expect("Couldn't read api key");

        Ok(key)
    }

    /// Retrieves current username - Useful for setting the path to store the keys.
    pub fn get_user() -> String {
        let user = get_user_by_uid(get_current_uid()).unwrap();
        let username = user.name().to_string_lossy();
        username.to_string()
    }
}

/// For handling different request timings. Known overflow errors at the moment, so use with caution. Use manual dates if possible.
///
/// # Query in a one month range
/// ```
/// use voyager_client::timing;
/// 
/// let start_date = timing::one_month();
/// let end_date = timing::today();
/// ```
/// # Query in a week range
/// ```
/// use voyager_client::timing;
/// 
/// let start_date = timing::one_week();
/// let end_date = timing::today();
/// ```
pub mod timing {
    use chrono::prelude::*;

    /// Returns the current date in YYYY-MM-DD format as a String
    /// # Example
    /// ```
    /// use voyager_client::timing;
    /// 
    /// let today = timing::today();
    ///
    /// ```
    pub fn today() -> String {
        let local: DateTime<Local> = Local::now();
        let today = format!("{}-{}-{}", local.year(), local.month(), local.day());
        today
    }

    /// Returns yesterday's date. Useful for queries that return lots of data daily and only data from
    /// the last 24h is needed.
    pub fn one_day() -> String {
        let local: DateTime<Local> = Local::now();
        let start = format!("{}-{}-{}", local.year(), local.month(), local.day() - 1);
        start
    }

    /// Returns the date exactly one week ago from today.
    pub fn one_week() -> String {
        let local: DateTime<Local> = Local::now();
        let start = format!("{}-{}-{}", local.year(), local.month(), local.day() - 7);
        start
    }

    /// Returns the date exactly two weeks ago from today
    pub fn two_weeks() -> String {
        let local: DateTime<Local> = Local::now();
        let start = format!("{}-{}-{}", local.year(), local.month(), local.day() - 14);
        start
    }

    /// Returns the date exactly one month ago from today
    pub fn one_month() -> String {
        let local: DateTime<Local> = Local::now();
        if local.month() == 1 {
            let last_month = 12;
            let start = format!("{}-{}-{}", local.year() - 1, last_month, local.day());
            start
        } else {
            let start = format!("{}-{}-{}", local.year(), local.month() - 1, local.day());
            start
        }
    }
}

/// Contains methods for prettyfying JSON responses.
///
/// Will output a prettyfied String (JSON format) response instead of a large unformatted JSON blob
///
pub mod to_pretty {
    use serde_json::Value as JsonValue;
    use std::error::Error;

    /// Converts a JSON blob into a pretty string, easily readible.
    pub fn to_string_pretty(res: String) -> Result<String, Box<dyn Error>> {
        let json: JsonValue = serde_json::from_str(&res).unwrap();
        let pretty: String = serde_json::to_string_pretty(&json).unwrap();

        Ok(pretty)
    }
}

/// For interacting with NASA's Picture of the Day endpoint.
///
/// # Querying APOD endpoint
///
/// ```
/// use voyager_client::{apod_client, timing};
/// 
/// // Instantiate the base client
/// let mut base = apod_client::Apod::new();
/// // Set the date for query
/// base.set_date(String::from("2022-01-07"));
/// // Query the endpoint
/// let res = base.query().unwrap();
/// ```
/// This will return a response containing data on the Picture of the Day, as well as the url to the jpg file.
///
pub mod apod_client {
    use std::error::Error;

    use super::keys;
    use super::to_pretty::to_string_pretty;

    pub struct Apod {
        base_url: String,
        date: Option<String>,
    }

    impl Apod {
        pub fn new() -> Self {
            Apod {
                base_url: String::from("https://api.nasa.gov/planetary/apod?"),
                date: None,
            }
        }

        pub fn set_date(&mut self, date: String) {
            self.date = Some(date);
        }

        pub fn query(&self) -> Result<String, Box<dyn Error>> {
            let key: String = keys::get_key()?;

            if self.date.is_none() {
                let url = format!("{}api_key={}", self.base_url, key);

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
}

/// Contains methods for interacting with the DONKI client library.
///
/// # Querying solar flare API
///
/// ```
/// use voyager_client::{donki_client, timing};
/// 
/// // Instantiate Base Client
/// let mut base = donki_client::Solar::new();
///
/// // Setup Timing
/// let start = timing::one_month();
/// let end = timing::today();
/// // Query Endpoint
/// let res = base.query(start, end).unwrap();
///
/// ```
/// all API query functions will pipe out their response into the progress bar method
/// which will in turn print the response after it's finished processing.
///
/// # Querying magnetic storm endpoints
///
/// ```
/// use voyager_client::{donki_client, timing};
/// 
/// // Setup timing
/// let start = String::from("2019-01-01");
/// let end = String::from("2022-01-01");
///
/// // Instantiate Base Client
///  let mut base = donki_client::Magnetic::new();
///
/// // Query Endpoint
/// let res = base.query(start, end).unwrap();
///
/// ```
///
pub mod donki_client {
    use std::error::Error;

    use super::keys;
    use super::to_pretty::to_string_pretty;

    pub struct Solar {
        base_url: String,
    }

    impl Solar {
        pub fn new() -> Self {
            Solar {
                base_url: String::from("https://api.nasa.gov/DONKI/FLR?startDate="),
            }
        }

        pub fn query(&self, start: String, end: String) -> Result<String, Box<dyn Error>> {
            let key: String = keys::get_key()?;

            let url: String = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
            println!("Starting solar query from {}, to {}.", start, end);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let solar = to_string_pretty(res).unwrap();

            Ok(solar)
        }
    }

    pub struct Magnetic {
        base_url: String,
    }

    impl Magnetic {
        pub fn new() -> Self {
            Magnetic {
                base_url: String::from("https://api.nasa.gov/DONKI/GST?startDate="),
            }
        }

        pub fn query(&self, start: String, end: String) -> Result<String, Box<dyn Error>> {
            let key: String = keys::get_key()?;

            let url: String = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
            println!("Starting magnetic query from {}, to {}.", start, end);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let mag = to_string_pretty(res).unwrap();

            Ok(mag)
        }
    }

    /// For interacting with the Coronal Mass Ejction API.
    ///
    /// # Example
    /// ```
    /// use voyager_client::{donki_client, timing};
    /// 
    /// // Instantiate Base Client
    /// let base = donki_client::CoronalMassEjection::new();
    ///
    /// /// // Setup Timings
    /// let start = String::from("2022-01-01");
    /// let end = String::from("2022-01-07");
    ///
    /// // Query Endpoint
    /// let res = base.query(start, end).unwrap();
    /// ```
    pub struct CoronalMassEjection {
        base_url: String,
    }

    impl CoronalMassEjection {
        pub fn new() -> Self {
            CoronalMassEjection {
                base_url: String::from("https://api.nasa.gov/DONKI/CME?startDate="),
            }
        }

        pub fn query(&self, start: String, end: String) -> Result<String, Box<dyn Error>> {
            let key = keys::get_key()?;

            let url = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
            println!("Starting CME query from {}, to {}.", start, end);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let cme = to_string_pretty(res).unwrap();

            Ok(cme)
        }
    }
}

/// For interacting with the Near Earth Objects API.
///
/// # Example
/// ```
/// use voyager_client::{neo_client, timing};
/// 
/// // Instantiate Base Client
/// let base = neo_client::Neo::new();
///
/// // Setup Timings
/// let start = String::from("2022-01-01");
/// let end = String::from("2022-01-07");
///
/// // Query Endpoint
/// let res = base.query(start, end).unwrap();
/// ```
/// Neo currently reccomends a one day query, as it's database is constantly being updated
/// due to the nature of the data. Any query greater than a month is likely to take a long time to
/// process.
pub mod neo_client {
    use std::error::Error;

    use super::keys;
    use super::to_pretty::to_string_pretty;

    pub struct Neo {
        base_url: String,
    }

    impl Neo {
        pub fn new() -> Self {
            Neo {
                base_url: String::from("https://api.nasa.gov/neo/rest/v1/feed?start_date="),
            }
        }

        pub fn query(&self, start: String, end: String) -> Result<String, Box<dyn Error>> {
            let key: String = keys::get_key()?;

            let url: String = format!(
                "{}{}&endDate={}&api_key={}",
                self.base_url, start, end, key
            );
            println!("Starting Neo query from {}, to {}.", start, end);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let neo = to_string_pretty(res).unwrap();

            Ok(neo)
        }
    }
}

/// For interacting with the Insight Rover API.
///
/// # Example
/// ```
/// use voyager_client::{insight, timing};
/// 
/// // Instantiate Base Client
/// let base = insight::InsightWeather::new();
///
/// // Setup Timing Params
/// let start = String::from("2021-01-01");
/// let end = timing::today();
///
/// // Query Endpoint
/// let res = base.query().unwrap();
/// ```
pub mod insight {
    use std::error::Error;

    use super::keys;
    use super::to_pretty::to_string_pretty;

    pub struct InsightWeather {
        base_url: String,
    }

    impl InsightWeather {
        pub fn new() -> Self {
            InsightWeather {
                base_url: String::from("https://api.nasa.gov/insight_weather/?api_key="),
            }
        }

        pub fn query(&self) -> Result<String, Box<dyn Error>> {
            let key = keys::get_key()?;

            let url = format!("{}{}&feedtype=json&ver=1.0", self.base_url, key);
            println!("Starting Inisght Weather query: {}", url);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let mrover = to_string_pretty(res).unwrap();

            Ok(mrover)
        }
    }
}
