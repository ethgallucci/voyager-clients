//!
//! # Sample program with voyager_client
//!
//! Create a .env file at the root of your project
//! and add your api key with the variable name API_KEY. Make sure to add .env to your .gitignore!
//!
//! ```
//! use voyager_client::{donki_client, timing};
//!
//! // Instantiate a Base Client
//! let base = donki_client::CoronalMassEjection::new();
//!
//! // Setup timing parameters
//! let start = String::from("2020-01-01");
//! let end = timing::today();
//!
//! // Query the API
//! base.query(start, end).unwrap();
//! ```
//! This will fetch a response from the magnetic storms endpoint, and convert it
//! into a prettyfied String in JSON format

#![allow(dead_code)]
#![forbid(unsafe_code)]
/// Handling API keys for NASA's open APIs from .env files.
///  All keys must be stored in a .env file in the root directory of your project with the key "API_KEY".
///
/// # Retrieving a key
/// ```
/// use voyager_client::keys;
///
/// let key = keys::from_dotenv().unwrap();
/// ```

pub mod keys {
    use dotenv;
    use std::error::Error;

    pub fn from_dotenv() -> Result<String, Box<dyn Error>> {
        dotenv::dotenv().ok();

        let key = "API_KEY";
        let value = dotenv::var(key)?;
        Ok(value)
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

    #[derive(Debug, PartialEq)]
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
            let key: String = keys::from_dotenv()?;

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

    #[derive(Debug, PartialEq)]
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
            let key: String = keys::from_dotenv()?;

            let url: String = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
            println!("Starting solar query from {}, to {}.", start, end);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let solar = to_string_pretty(res).unwrap();

            Ok(solar)
        }
    }

    #[derive(Debug, PartialEq)]
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
            let key: String = keys::from_dotenv()?;

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

    #[derive(Debug, PartialEq)]
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
            let key = keys::from_dotenv()?;

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

    #[derive(Debug, PartialEq)]
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
            let key: String = keys::from_dotenv()?;

            let url: String = format!("{}{}&endDate={}&api_key={}", self.base_url, start, end, key);
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
/// use voyager_client::{insight_client, timing};
///
/// // Instantiate Base Client
/// let base = insight_client::InsightWeather::new();
///
/// // Setup Timing Params
/// let start = String::from("2021-01-01");
/// let end = timing::today();
///
/// // Query Endpoint
/// let res = base.query().unwrap();
/// ```
pub mod insight_client {
    use std::error::Error;

    use super::keys;
    use super::to_pretty::to_string_pretty;

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

        pub fn query(&self) -> Result<String, Box<dyn Error>> {
            let key = keys::from_dotenv()?;

            let url = format!("{}{}&feedtype=json&ver=1.0", self.base_url, key);
            println!("Starting Inisght Weather query: {}", url);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let mrover = to_string_pretty(res).unwrap();

            Ok(mrover)
        }
    }
}

/// For interacting with the Tech Transfer API
/// Defaults to the patent collection
/// Can also be switched to software via the .software() method
///
/// # Example
/// ```
/// use voyager_client::tech_transfer::*;
///
/// let mut base = TechTransferClient::new();
///
/// // Default collection is patents, can switch to software
/// base.switch(Collections::Software).unwrap();
///
/// let query = String::from("engine");
/// base.query(query).unwrap();
/// ```

pub mod tech_transfer {
    use std::error::Error;

    use super::keys;
    use super::to_pretty::to_string_pretty;

    pub enum Collections {
        Patent,
        PatentIssued,
        Software,
        Spinoff,
    }

    #[derive(Debug, PartialEq)]
    pub struct TechTransferClient {
        base_url: String,
    }

    impl TechTransferClient {
        pub fn new() -> Self {
            TechTransferClient {
                base_url: String::from("https://api.nasa.gov/techtransfer/patent/?"),
            }
        }

        /// Switches Collection
        pub fn switch(&mut self, collection: Collections) -> Result<(), Box<dyn Error>> {
            match collection {
                Collections::Patent => {
                    self.base_url = String::from("https://api.nasa.gov/techtransfer/patent/?");
                    Ok(())
                }
                Collections::PatentIssued => {
                    self.base_url =
                        String::from("https://api.nasa.gov/techtransfer/patent_issued/?");
                    Ok(())
                }
                Collections::Software => {
                    self.base_url = String::from("https://api.nasa.gov/techtransfer/software/?");
                    Ok(())
                }
                Collections::Spinoff => {
                    self.base_url = String::from("https://api.nasa.gov/techtransfer/spinoff/?");
                    Ok(())
                }
            }
        }

        pub fn query(&self, query: String) -> Result<String, Box<dyn Error>> {
            let key: String = keys::from_dotenv()?;

            let url = format!("{}{}&api_key={}", self.base_url, query, key);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let tech = to_string_pretty(res);

            if tech.is_ok() {
                Ok(tech.unwrap())
            } else {
                Err(tech.unwrap_err())
            }
        }
    }
}

/// Jet Propulsion Laboratory
pub mod jpl {
    use std::error::Error;

    use super::to_pretty::to_string_pretty;

    /// Atmospheric Impact Data
    /// # Example usage
    /// ```
    /// use voyager_client::jpl;
    ///
    /// let mut base = jpl::FireballClient::new();
    /// // Optionally limit the number of responses
    /// base.limit(10);
    ///
    /// base.query().unwrap();
    /// ```

    #[derive(Debug, PartialEq)]
    pub struct FireballClient {
        base_url: String,
        limit: Option<u32>,
    }

    impl FireballClient {
        pub fn new() -> Self {
            FireballClient {
                base_url: String::from("https://ssd-api.jpl.nasa.gov/fireball.api"),
                limit: None,
            }
        }

        pub fn limit(&mut self, limit: u32) {
            self.limit = Some(limit)
        }

        pub fn query(&self) -> Result<String, Box<dyn Error>> {
            if self.limit.is_none() {
                let url = format!("{}", self.base_url);

                let res: String = ureq::get(&url).call()?.into_string()?;
                let fireball = to_string_pretty(res).unwrap();
                Ok(fireball)
            } else {
                let limit = self.limit.as_ref().unwrap();

                let url = format!("{}?limit={}", self.base_url, limit);

                let res: String = ureq::get(&url).call()?.into_string()?;
                let fireball = to_string_pretty(res).unwrap();

                Ok(fireball)
            }
        }
    }

    /// Base Client for the JPL Mission Design API in Query Mode.
    /// # Example
    /// ```
    /// use voyager_client::jpl::{MissionDesign, QueryType};
    ///
    /// let mut base = MissionDesign::new();
    ///
    /// base.query(QueryType::DES, "2012%20TC4").unwrap();
    /// ```
    ///
    
    /// Mission Design in Q Mode (query)
    #[derive(Debug, PartialEq)]
    pub struct MissionDesign {
        base_url: String,
    }

    #[derive(Debug, PartialEq)]
    pub enum QueryType {
        /// designation (provisional or IAU-number) of the desired object (e.g., 2015 AB or 141P or 433).
        /// NOTE: when submitting a des containing a space in your query string, you must replace the space with %20, for example 2015%20AB.
        DES,
        /// object search string: designation in various forms (including MPC packed form), case-insensitive name, or SPK-ID;
        /// designation can be an alternate provisional designation; examples: atira, 2003 CP20, 2003cp20, K03C20P, 163693, 2163693
        SSTR,
    }

    impl MissionDesign {
        pub fn new() -> Self {
            MissionDesign {
                base_url: String::from("https://ssd-api.jpl.nasa.gov/mdesign.api?"),
            }
        }

        /// Mission Design: Q mode (query)
        pub fn query(&self, query_type: QueryType, query: &str) -> Result<String, Box<dyn Error>> {
            // Default Query Mode
            match query_type {
                QueryType::DES => {
                    let url = format!("{}des={}", self.base_url, query);

                    let res: String = ureq::get(&url).call()?.into_string()?;
                    let mission = to_string_pretty(res).unwrap();

                    Ok(mission)
                }
                QueryType::SSTR => {
                    let url = format!("{}sstr={}", self.base_url, query);

                    let res: String = ureq::get(&url).call()?.into_string()?;
                    let mission = to_string_pretty(res).unwrap();

                    Ok(mission)
                }
            }
        }
    }

    /// Base Client for Mission Design in Accessible Mode (A)
    /// # Example
    /// ```
    /// use voyager_client::jpl::*;
    ///
    /// let mut base = MissionDesignAccessible::new();
    /// base.limit(10);
    /// base.crit(1);
    /// base.year(String::from("2025,2026,2027,2028,2029"));
    /// 
    /// base.lim_crit_year().unwrap();
    /// ```
    #[derive(Debug, PartialEq)]
    pub struct MissionDesignAccessible {
        base_url: String,
        limit: Option<u32>,
        crit: Option<u8>,
        year: Option<String>,
        rdvz: Option<bool>,
        class: Option<String>
    }

    impl MissionDesignAccessible {
        /// Create a new MissionDesignAccessible base client with None set for the limit, crit, year, rdvz and class fields
        pub fn new() -> Self {
            MissionDesignAccessible {
                base_url: String::from("https://ssd-api.jpl.nasa.gov/mdesign.api?"),
                limit: None,
                crit: None,
                year: None,
                rdvz: None,
                class: None,
            }
        }

        pub fn limit(&mut self, limit: u32) {
            self.limit = Some(limit)
        }

        pub fn crit(&mut self, crit: u8) {
            self.crit = Some(crit)
        }

        pub fn year(&mut self, year: String) {
            self.year = Some(year)
        }

        pub fn rdvz(&mut self, rdvz: bool) {
            self.rdvz = Some(rdvz)
        }

        pub fn class(&mut self, class: String) {
            self.class = Some(class)
        }

        /// Must set Limit, Crit, and year values
        pub fn lim_crit_year(&self) -> Result<String, Box<dyn Error>> {
            assert!(self.limit != None, "Limit is None");
            assert!(self.crit != None, "Crit is None");
            assert!(self.year != None, "Year is None");

            let url = format!(
                "{}lim={}&crit={}&year={}",
                self.base_url, self.limit.as_ref().unwrap(), self.crit.as_ref().unwrap(), self.year.as_ref().unwrap()
            );

            let res = ureq::get(&url).call()?.into_string()?;
            let pretty = to_string_pretty(res).unwrap();

            Ok(pretty)
        }
    }

    /// Base Client for Mission Design in Map Mode (M)
    /// # Example
    /// ```
    /// use voyager_client::jpl::*;
    ///
    /// let mut base = MissionDesignMap::new();
    /// base.designation("2012%20TC4");
    /// base.mjd(58490);
    /// base.span(3652);
    /// base.tof(10, 36);
    /// base.step(2);
    /// 
    /// base.query().unwrap();;
    /// ```
    #[derive(Debug, PartialEq)]
    pub struct MissionDesignMap {
        base_url: String,
        des: Option<String>,
        mjd0: Option<u32>,
        span: Option<u32>,
        tof_min: Option<u32>,
        tof_max: Option<u32>,
        step: Option<u8>
    }

    impl MissionDesignMap {
        pub fn new() -> Self {
            MissionDesignMap {
                base_url: String::from("https://ssd-api.jpl.nasa.gov/mdesign.api?"),
                des: None,
                mjd0: None,
                span: None,
                tof_min: None,
                tof_max: None,
                step: None
            }
        }

        pub fn designation(&mut self, des: &str) {
            self.des = Some(String::from(des))
        }

        pub fn mjd(&mut self, x: u32) {
            self.mjd0 = Some(x)
        }

        pub fn span(&mut self, span: u32) {
            self.span = Some(span)
        }

        pub fn tof(&mut self, min: u32, max: u32) {
            self.tof_min = Some(min);
            self.tof_max = Some(max);
        }

        pub fn step(&mut self, step: u8) {
            self.step = Some(step)
        }

        pub fn query(&self) -> Result<String, Box<dyn Error>> {
            assert!(self.des != None, "Des is None");
            assert!(self.mjd0 != None, "Mjd0 is None");
            assert!(self.span != None, "Span is None");
            assert!(self.tof_min != None, "tof_min is None");
            assert!(self.tof_max != None, "tof_max is None");
            assert!(self.step != None, "Step is None");

            let url = format!(
                "{}des={}&mjd0={}&span={}&tof-min={}&tof-max={}&step={}",
                self.base_url, self.des.as_ref().unwrap(), 
                self.mjd0.as_ref().unwrap(), self.span.as_ref().unwrap(),
                self.tof_min.as_ref().unwrap(), self.tof_max.as_ref().unwrap(),
                self.step.as_ref().unwrap()
            );

            println!("Url: {}", url);

            let res: String = ureq::get(&url).call()?.into_string()?;
            let map = to_string_pretty(res).unwrap();

            Ok(map)
        }
    }
}
