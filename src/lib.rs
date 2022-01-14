#![warn(rustdoc::broken_intra_doc_links)]

//! # API bindings for NASA's Open APIs
//! Features light bindings for a multitude of APIs. Including:
//! * Picture of the day -> [`apod`]
//! * Database of Notifications, Knowledge, Information -> [`donki`]  
//! * Jet Propulsion Laboratory -> [`jpl`]
//! * Near Earth Objects -> [`neo`]
//! * Tech Transfer -> [`tech`]

//! # Features
//! * Base clients for interacting with NASA's open APIs
//! * Methods for easily turning responses into serde_json values
//! * Each client is it's own struct that holds at least one field (base_url: String). Some clients
//! that take more complex query params may have a field for each parmater, and include methods to
//! update the params before sending the query.

//! # Sample program with the GeoMagnetic base client
//! Create a .env file at the root of your project
//! and add your api key with the variable name API_KEY.
//!
//! ```
//! use voyager_client::{donki, time};
//! use voyager_client::response::*;
//! 
//! use serde_json::Value as JsonValue;
//!
//! // Instantiate a base client
//! let base = donki::GeoMagnetic::new();
//! 
//! // Setup time
//! let start = String::from("2015-01-01");
//! let end = time::today();
//! // Query the endpoint
//! let res: Response = base.query(start, end).unwrap();
//! 
//! // Manipulating the response
//! let json: JsonValue = res.json().unwrap();
//! let bytes_vec: Vec<u8> = res.bytedump().unwrap();
//! ```
//! Base clients query methods return a Response object. 

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


/// Contains methods for interacting with the DONKI client library.
///
/// # Querying solar flare API
///
/// ```
/// use voyager_client::donki;
/// use voyager_client::time;
///
/// // Instantiate Base Client
/// let mut base = donki::SolarFlare::new();
///
/// // Setup time
/// let start = time::one_month();
/// let end = time::today();
/// // Query Endpoint
/// let res = base.query(start, end).unwrap();
///
/// ```
///
/// # Querying magnetic storm endpoints
///
/// ```
/// use voyager_client::donki;
///
/// // Setup time
/// let start = String::from("2019-01-01");
/// let end = String::from("2022-01-01");
///
/// // Instantiate Base Client
///  let mut base = donki::GeoMagnetic::new();
///
/// // Query Endpoint
/// let res = base.query(start, end).unwrap();
///
/// ```
pub mod donki;


/// Handling API keys for NASA's open APIs from .env files.
///  All keys must be stored in a .env file in the root directory of your project with the key "API_KEY".
///
/// # Retrieving a key
/// ```
/// use voyager_client::key;
///
/// let key = key::from_dotenv().unwrap();
/// ```
pub mod key;


/// Contains methods for prettyfying JSON responses.
pub mod pretty;
pub use pretty::*;

/// For interacting with the Near Earth Objects API.
///
/// # Example
/// ```
/// use voyager_client::{neo, time};
///
/// // Instantiate Base Client
/// let base = neo::Neo::new();
///
/// // Setup times
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


/// For interacting with the Insight Rover API.
///
/// # Example
/// ```
/// use voyager_client::insight;
///
/// // Instantiate Base Client
/// let base = insight::InsightWeather::new();
///
/// // Query Endpoint
/// let res = base.query().unwrap();
/// ```
pub mod insight;


/// Methods for manipulating responses. Get serde_json dumps or byte vectors from responses.
/// # Example
/// ```
/// use voyager_client::donki::*;
/// use serde_json::Value as JsonValue;
///
/// // Setup range
/// let start = String::from("2019-01-01");
/// let end = String::from("2022-01-01");
/// // Instantiate base
/// let base = GeoMagnetic::new();
/// // Try query
/// let res = base.query(start, end).unwrap();
/// // Handle Response
/// let json: JsonValue = res.json().unwrap();
/// let bytes_vec: Vec<u8> = res.bytedump().unwrap();
/// ```
pub mod response;

/// For interacting with the Tech Transfer API. Defaults to the patent collection but can 
/// also be switched to patent protected, software, or spinoff via the .switch() method.
///
/// # Example
/// ```
/// use voyager_client::tech;
/// use tech::Collections;
///
/// let mut base = tech::TechTransferClient::new();
///
/// // Default collection is patents, can switch to software
/// base.switch(Collections::Software).unwrap();
///
/// let query = String::from("engine");
/// base.query(query).unwrap();
/// ```
pub mod tech;


/// Jet Propulsion Laboratory
/// # Example usage with FireballClient
/// ```
/// use voyager_client::jpl::*;
///
/// // Instantiate Base Client
/// let mut base = FireballClient::new();
/// 
/// // Optionally limit the number of responses
/// base.limit(10);
///
/// base.query().unwrap();
/// ```
/// 
/// # Example usage with MissionDesign
/// ```
/// use voyager_client::jpl::*;
/// 
/// // Instantiate Base Client
/// let mut base = MissionDesign::new();
/// 
/// base.query(QueryType::DES, "2012%20TC4").unwrap();
/// ```
/// 
pub mod jpl;


/// For handling different request times. Known overflow errors at the moment, so use with caution. Use manual dates if possible.
///
/// # Query in a one month range
/// ```
/// use voyager_client::time;
///
/// let start_date = time::one_month();
/// let end_date = time::today();
/// ```
/// # Query in a week range
/// ```
/// use voyager_client::time;
///
/// let start_date = time::one_week();
/// let end_date = time::today();
/// ```
pub mod time;


