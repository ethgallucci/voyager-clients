//!
//! # Sample Program with voyager
//! ```
//! use voyager_client::*;
//!
//! // Run this function only once
//! keys::set_key("[YOUR_API_KEY]");
//!
//! let magnetic_storms = weather::magnetic().unwrap();
//! ```
//! This will fetch a response from the magnetic storms endpoint, and convert it
//! into a prettyfied String in JSON format

#![allow(dead_code)]

pub use bar::*;
pub use keys::*;
pub use neo_client::*;
pub use timing::*;
pub use to_pretty::*;
pub use donki_client::*;
pub use apod_client::*;

/// Handling API keys for NASA's open APIs.
/// Includes methods for storing, and retrieving keys for any user
///
/// # Configuring a key
/// ```
/// set_key("[YOUR_API_KEY]")?;
/// ```
///
/// # Retrieving a key
/// ```
/// let key = get_key()?;
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

/// For handling different request timings. Methods for formatting one day requests, one week,
/// two week, and one month requests into a String format.
///
/// # Query in a one month range
/// ```
/// let start_date = timing::one_month();
/// let end_date = timing::today();
/// ```
/// # Query in a week range
/// ```
/// let start_date = timing::one_week();
/// let end_date = timing::today();
/// ```
pub mod timing {
    use chrono::prelude::*;

    /// Returns the current date in YYYY-MM-DD format as a String
    /// # Example
    /// ```
    /// let today = timing::today();
    ///
    /// assert_eq!("2021-12-21", today);
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
        }
        else {
            let start = format!("{}-{}-{}", local.year(), local.month() - 1, local.day());
            start
        }
    }
}

/// Displaying progress bar in the terminal as part of the CLI
///
/// # Setup a progress bar
/// ```
/// let byte_length = response.as_bytes().len();
/// bar::bar(byte_length);
/// ```
pub mod bar {
    extern crate pbr;
    use pbr::ProgressBar;

    /// Takes a response length (in bytes) as its only argument
    pub fn bar(res: &String) -> () {
        let count = res.len();
        let mut pb = ProgressBar::new(count as u64);
        pb.format("╢▌▌-╟");
        pb.show_percent = true;
        pb.show_time_left = true;
        for _ in 0..count {
            pb.inc();
        }
        pb.finish_print("done!");
        println!("{}", res);
    }
}

/// Contains methods for prettyfying JSON responses.
///
/// # Example
///
/// ```
/// let res: String = ureq::get(&url).call()?.into_string()?;
/// let neo = to_string_pretty(res).unwrap();
/// ```
/// This will output a prettyfied String (JSON format) response instead of a large unformatted JSON blob
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
/// // Instantiate the base client
/// let mut base = apod::new();
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
            }
            else {
                let date = self.date.as_ref().unwrap();
                let url = format!("{}date={}&api_key={}", self.base_url, date, key);

                let res: String = ureq::get(&url).call()?.into_string()?;
                let apod = to_string_pretty(res).unwrap();

                Ok(apod)
            }
        }
    }
}

/// Contains methods for interacting with Solar Flare and Magnetic Storm endpoints.
///
/// # Querying solar flare API
///
/// ```
/// // Setup Timing
/// let start = timing::one_month();
/// let end = timing::today();
/// 
/// // Instantiate Base Client
/// let mut base = Solar::new(start, end);
/// 
/// // (Optional) Update Timings
/// base.set_start(String::from("2022-01-02"));
/// 
/// // Query Endpoint
/// let res = base.query().unwrap();
/// 
/// ```
/// all API query functions will pipe out their response into the progress bar method
/// which will in turn print the response after it's finished processing.
///
/// # Querying magnetic storm endpoints
///
/// ```
/// // Setup Timing
/// --snip--
/// 
/// // Instantiate Base Client
///  let mut base = Magnetic::new(start, end);
/// 
/// // Query Endpoint
/// let res = base.query().unwrap();
/// 
/// ```
///
pub mod donki_client {
    use std::error::Error;

    use super::to_pretty::to_string_pretty;
    use super::keys;

    pub struct Solar {
        base_url: String,
        pub start: String,
        pub end: String,
    }

    impl Solar {
        pub fn new(start: String, end: String) -> Self {
            Solar{
                base_url: String::from("https://api.nasa.gov/DONKI/FLR?startDate="),
                start,
                end,
            }
        }

        pub fn set_start(&mut self, start: String) {
            self.start = start;
        }

        pub fn set_end(&mut self, end: String) {
            self.end = end;
        }

        pub fn query(&self) -> Result<String, Box<dyn Error>> {
            let key: String = keys::get_key()?;

            let url: String = format!(
                "{}{}&endDate={}&api_key={}",
                self.base_url, self.start, self.end, key
            );
            println!("Starting solar query from {}, to {}.", self.start, self.end);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let solar = to_string_pretty(res).unwrap();

            Ok(solar)
        }
    }

    pub struct Magnetic {
        base_url: String,
        start: String,
        end: String,
    }

    impl Magnetic {
        pub fn new(start: String, end: String) -> Self {
            Magnetic {
                base_url: String::from("https://api.nasa.gov/DONKI/GST?startDate="),
                start,
                end,
            }
        }

        pub fn set_start(&mut self, start: String) {
            self.start = start;
        }

        pub fn set_end(&mut self, end: String) {
            self.end = end;
        }

        pub fn query(&self) -> Result<String, Box<dyn Error>> {
            let key: String = keys::get_key()?;

            let url: String = format!(
                "{}{}&endDate={}&api_key={}",
                self.base_url, self.start, self.end, key
            );
            println!("Starting magnetic query from {}, to {}.", self.start, self.end);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let mag = to_string_pretty(res).unwrap();

            Ok(mag)
        }
    }
}

/// For interacting with the Near Earth Objects API.
///
/// # Example
/// ```
/// // Setup Timings
/// let start = String::from("2022-01-01");
/// let end = String::from("2022-01-07");
/// 
/// // Instantiate Base Client
/// let base = Neo::new(start, end);
/// 
/// // Query Endpoint
/// let res = base.query().unwrap();
/// ```
/// Neo currently reccomends a one day query, as it's database is constantly being updated
/// due to the nature of the data. Any query greater than a month is likely to take a long time to
/// process.
pub mod neo_client {
    use std::error::Error;

    use super::to_pretty::to_string_pretty;
    use super::keys;

    pub struct Neo {
        base_url: String,
        start: String,
        end: String,
    }

    impl Neo {
        pub fn new(start: String, end: String) -> Self {
            Neo {
                base_url: String::from("https://api.nasa.gov/neo/rest/v1/feed?start_date="),
                start,
                end
            }
        }

        pub fn set_start(&mut self, start: String) {
            self.start = start;
        }

        pub fn set_end(&mut self, end: String) {
            self.end = end;
        }

        pub fn query(&self) -> Result<String, Box<dyn Error>> {
            let key: String = keys::get_key()?;

            let url: String = format!(
                "{}{}&endDate={}&api_key={}",
                self.base_url, self.start, self.end, key
            );
            println!("Starting Neo query from {}, to {}.", self.start, self.end);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let neo = to_string_pretty(res).unwrap();

            Ok(neo)
        }
    }
}
