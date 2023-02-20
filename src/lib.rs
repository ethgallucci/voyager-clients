//! # NASA Open APIs client implementations.
//! Features light bindings for a multitude of APIs. Including:
//! * Picture of the day -> [`apod`]
//! * Database of Notifications, Knowledge, Information -> [`donki`]  
//! * Jet Propulsion Laboratory -> [`jpl`]
//! * Near Earth Objects -> [`neo`]
//! * Tech Transfer -> [`tech`]
//! ## Example Program
//! ```no_run
//! use nerva::clients::apod;
//! use nerva::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>>
//! {
//!     let apod = apod::Apod::default();
//!     let response = apod.get(params::ApodParams::default())?;
//!     println!("{:#?}", response);
//!     return Ok(());
//! }
//! ```

#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]
/// API client implementations
pub mod clients;
/// Core abstractions
pub mod core;
/// Re-exports
pub mod prelude;
