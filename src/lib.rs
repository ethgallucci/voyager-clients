//! # API bindings for NASA's Open APIs
//! Features light bindings for a multitude of APIs. Including:
//! * Picture of the day -> [`apod`]
//! * Database of Notifications, Knowledge, Information -> [`donki`]  
//! * Jet Propulsion Laboratory -> [`jpl`]
//! * Near Earth Objects -> [`neo`]
//! * Tech Transfer -> [`tech`]

#![warn(missing_docs)]
#![feature(stmt_expr_attributes)]
pub use self::prelude::*;

#[doc = "Util methods for API keys"]
#[allow(dead_code)]
pub(crate) mod key {
    pub fn load(k: &str) -> String {
        dotenv::dotenv().ok();
        std::env::var(k).expect(&format!("{} is not set", k))
    }
}

#[doc = "Common interfaces"]
pub mod prelude;

#[doc = "Implementations of various NASA Open APIs"]
pub mod apis;
