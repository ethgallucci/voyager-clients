#![warn(rustdoc::broken_intra_doc_links)]

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

/// For interacting with NASA's Picture of the Day endpoint.
///
/// # Querying APOD endpoint
///
/// ```
/// use voyager_client::apod;
///
/// // Instantiate the base client
/// let mut base = apod::ApodClient::new();
/// // Set the date for query
/// base.set_date(String::from("2022-01-07"));
/// // Query the endpoint
/// let res = base.query().unwrap();
/// ```
/// This will return a response containing data on the Picture of the Day, as well as the url to the jpg file.
pub mod apod;
pub use apod::*;


/// Contains methods for interacting with the DONKI client library.
///
/// # Querying solar flare API
///
/// ```
/// use voyager_client::donki_client;
/// use voyager_client::timing;
///
/// // Instantiate Base Client
/// let mut base = donki_client::SolarFlare::new();
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
pub mod donki;
pub use donki::*;


/// Handling API keys for NASA's open APIs from .env files.
///  All keys must be stored in a .env file in the root directory of your project with the key "API_KEY".
///
/// # Retrieving a key
/// ```
/// use voyager_client::keys;
///
/// let key = keys::from_dotenv().unwrap();
/// ```
pub mod key;
pub use key::keys;


/// Contains methods for prettyfying JSON responses.
/// Will output a prettyfied String (JSON format) response instead of a large unformatted JSON blob
pub mod to_pretty;
pub use to_pretty::*;


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
pub mod neo;
pub use neo::*;


/// For interacting with the Insight Rover API.
///
/// # Example
/// ```
/// use voyager_client::insight;
/// use voyager_client::timing;
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

pub mod insight;

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
pub mod tech;
pub use tech::tech_transfer;


/// Jet Propulsion Laboratory
/// # Example usage
/// ```
/// use voyager_client::jpl_client::*;
///
/// let mut base = FireballClient::new();
/// // Optionally limit the number of responses
/// base.limit(10);
///
/// base.query().unwrap();
/// ```

pub mod jpl;
pub use jpl::*;


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
pub mod time;
pub use time::timing;