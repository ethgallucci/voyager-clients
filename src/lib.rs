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

pub use keys::*;
pub use timing::*;
pub use bar::*;
pub use to_pretty::*;
pub use apod::*;
pub use weather::*;
pub use neo::*;

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
    use std::fs;
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;

    use users::{ get_user_by_uid, get_current_uid };

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
        let path_to_key = format!("/Users/{}/voyager/.api_key.txt", user.name().to_string_lossy());
        let key = fs::read_to_string(&path_to_key)
            .expect("Couldn't read api key");

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
        let start = format!("{}-{}-{}", local.year(), local.month() - 1, local.day());
        start
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
    use serde_json::{Value as JsonValue};
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
/// let apod = apod().unwrap();
/// println!("{}", apod)
/// ```
/// This will return a response containing data on the Picture of the Day, as well as the url to the jpg file.
/// the println! is not necessary, because each query method includes it's own print statement as part of the 
/// progress bar function call. 
/// 
pub mod apod {
    use std::error::Error;

    use super::{keys, bar};
    use super::to_pretty::to_string_pretty;

    pub fn get_apod() -> Result<String, Box<dyn Error>> {
        let key: String = keys::get_key()?;
        let url: String = format!("https://api.nasa.gov/planetary/apod?api_key={}", key);

        let res: String = ureq::get(&url).call()?.into_string()?;
        let apod = to_string_pretty(res).unwrap(); 

        Ok(apod)
    }
}

/// Contains methods for interacting with Solar Flare and Magnetic Storm endpoints.
/// 
/// # Querying solar flare API
/// 
/// ```
/// let sflare = sflare().unwrap();
/// ```
/// all API query functions will pipe out their response into the progress bar method
/// which will in turn print the response after it's finished processing.
/// 
/// # Querying magnetic storm endpoints
/// 
/// ```
/// let mag = magnetic().unwrap();
/// ```
/// 
pub mod weather {
    use std::error::Error;

    use super::{timing, keys, bar};
    use super::to_pretty::to_string_pretty;

    pub fn sflare() -> Result<String, Box<dyn Error>> {
        let now = timing::today();
        let start = timing::two_weeks();
        println!("Starting query from {} to {}", start, now);
        
        let key: String = keys::get_key().unwrap();
        let url: String = format!("https://api.nasa.gov/DONKI/FLR?startDate={}&endDate={}&api_key={}", start, now, key);
        
        let res: String = ureq::get(&url).call()?.into_string()?;
        let sflare = to_string_pretty(res).unwrap();
        
        Ok(sflare)
    }

    pub fn magnetic() -> Result<String, Box<dyn Error>> {
        
        let key: String = keys::get_key().unwrap();
        let url: String = format!("https://api.nasa.gov/DONKI/GST?startDate=2021-01-01&endDate=2021-12-10&api_key={}", key);
        
        let res: String = ureq::get(&url).call()?.into_string()?;
        let magnetic = to_string_pretty(res).unwrap();

        Ok(magnetic)
    }
}

/// For interacting with the Near Earth Objects API.
/// 
/// # Example
/// ```
/// let neo = neo().unwrap()
/// ```
/// Neo currently uses a one day query, as it's database is constantly being updated
/// due to the nature of the data. Any query greater than a week is likely to take a long time to
/// process.
pub mod neo {
    use std::error::Error;

    use super::{timing, keys, bar};
    use super::to_pretty::to_string_pretty;

    pub fn neo() -> Result<String, Box<dyn Error>> {
        let start = timing::one_day();
        let now = timing::today();
        println!("Starting query from {} to {}", start, now);
        
        let key = keys::get_key()?;
        let url = format!("https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}", start, now, key);
        
        let res: String = ureq::get(&url).call()?.into_string()?;
        let neo = to_string_pretty(res).unwrap();

        Ok(neo)
    }
}