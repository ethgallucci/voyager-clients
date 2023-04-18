//! # NASA Open APIs client implementations.
//! Features light bindings for a multitude of APIs. Including:
//! * Picture of the day -> [`apod`]
//! * Database of Notifications, Knowledge, Information -> [`donki`]  
//! * Jet Propulsion Laboratory -> [`jpl`]
//! * Near Earth Objects -> [`neo`]
//! * Tech Transfer -> [`tech`]
//! ## Example Program
//! use nerva::client::apod::*;
//!
//! use nerva::filter::{filter, Match};
//! use nerva::core::Filter;
//! use nerva::prelude::*;

//! fn main() {
//!     let apod = Apod::default();
//!     let response = apod.get(ApodParams::date("2023-02-21"));
//!     let values = filter::<Match<String>>(response.unwrap(), &Match::new("explanation"));
//!     assert!(values.is_ok());
//!     println!("{:#?}", values.unwrap());
//! }
//! ```

#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]
#![allow(incomplete_features)]
#![warn(missing_docs)]
/// API client implementations
pub mod clients;
/// Core abstractions
pub mod core;
/// Re-exports
pub mod prelude;
