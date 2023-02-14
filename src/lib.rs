//! # API bindings for NASA's Open APIs
//! Features light bindings for a multitude of APIs. Including:
//! * Picture of the day -> [`apod`]
//! * Database of Notifications, Knowledge, Information -> [`donki`]  
//! * Jet Propulsion Laboratory -> [`jpl`]
//! * Near Earth Objects -> [`neo`]
//! * Tech Transfer -> [`tech`]

#![warn(missing_docs)]
#![feature(associated_type_defaults)]
#![feature(type_alias_impl_trait)]
/// Defines the core types of the library.
pub mod core;
/// Exposes core interfaces for clients
pub mod prelude;

pub mod apod
{
    use super::core::{ClientBuilder, OpenApi, Params};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
    pub(super) enum ApodParams<'p>
    {
        /// The date of the APOD
        Date(&'p str),
        /// The date of the APOD
        DateRange(&'p str, &'p str),
    }

    impl<'p> Default for ApodParams<'p>
    {
        fn default() -> Self { Self::Date("2020-01-01") }
    }

    pub struct Apod {}

    impl Default for Apod
    {
        fn default() -> Self { Self {} }
    }

    impl<'p, P> OpenApi<P> for Apod
    where
        P: Params,
    {
        type Params = P;
        type Response = Result<reqwest::Response, reqwest::Error>;

        fn get(&self, params: Self::Params) -> Self::Response { todo!() }
        fn get_raw(&self, params: Self::Params) -> Self::Response { todo!() }
        fn get_raw_with_headers(
            &self,
            params: Self::Params,
            headers: reqwest::header::HeaderMap,
        ) -> Self::Response
        {
            todo!()
        }
    }
}
