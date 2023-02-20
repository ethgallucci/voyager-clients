use crate::core::{Client, Params};
use std::error::Error;

/// Retrieve a list of Asteroids based on their closest approach date to Earth
pub mod feed
{
    use super::{Client, Error, Params};

    /// Params for the Neo <feed> API
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum NeoFeedParams<'p>
    {
        StartDate(&'p str),
        EndDate(&'p str),
    }

    impl<'p> Into<String> for NeoFeedParams<'p>
    {
        fn into(self) -> String
        {
            match self
            {
                NeoFeedParams::StartDate(date) => format!("start_date={}", date),
                NeoFeedParams::EndDate(date) => format!("end_date={}", date),
            }
        }
    }

    impl<'p> Default for NeoFeedParams<'p>
    {
        fn default() -> Self { return NeoFeedParams::StartDate("2023-01-01") }
    }

    impl<'p> Params for NeoFeedParams<'p> {}

    /// Neo <feed> client
    #[derive(Clone, Debug)]
    pub struct NeoFeed {}

    impl NeoFeed
    {
        pub fn new() -> Self { return NeoFeed::default() }
    }

    impl Default for NeoFeed
    {
        fn default() -> Self { return NeoFeed {} }
    }

    impl<'p, PARAMS> Client<PARAMS> for NeoFeed
    where
        PARAMS: Params,
    {
        const BASE_URL: &'static str = "https://api.nasa.gov/neo/rest/v1/feed";
        type Response = serde_json::Value;

        fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>>
        where
            PARAMS: Params,
        {
            let base_url = <NeoFeed as Client<PARAMS>>::BASE_URL;
            let url_with_params = format!("{}?{}", base_url, params.into());
            let url_with_key = crate::prelude::keys::include(&url_with_params)?;
            let response = ureq::get(&url_with_key).call()?;
            let json = serde_json::json!(response.into_string()?);
            Ok(json)
        }
    }

    #[cfg(test)]
    mod tests
    {
        use super::{NeoFeed, NeoFeedParams};
        use crate::core::Client;

        #[test]
        fn test_neo()
        {
            let neo = NeoFeed::default();
            let params: String = NeoFeedParams::default().into();
            println!("params: {}", params);
            let response = neo.get(NeoFeedParams::default());
            match response
            {
                Ok(json) => println!("{:#?}", json),
                Err(e) =>
                {
                    println!("{}", e);
                    panic!();
                }
            }
        }
    }
}

/// Lookup a specific Asteroid based on its NASA JPL small body (SPK-ID) ID
pub mod lookup
{
    use super::{Client, Error, Params};

    /// Params for the Neo <lookup> API
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum NeoLookupParams<'p>
    {
        AsteroidID(&'p str),
    }

    impl<'p> Into<String> for NeoLookupParams<'p>
    {
        fn into(self) -> String
        {
            match self
            {
                NeoLookupParams::AsteroidID(id) => format!("asteroid_id={}", id),
            }
        }
    }

    impl<'p> Default for NeoLookupParams<'p>
    {
        fn default() -> Self { return NeoLookupParams::AsteroidID("2021277") }
    }

    impl<'p> Params for NeoLookupParams<'p> {}

    /// Neo <lookup> client
    #[derive(Clone, Debug)]
    pub struct NeoLookup {}

    impl Default for NeoLookup
    {
        fn default() -> Self { return NeoLookup {} }
    }

    impl NeoLookup
    {
        pub fn new() -> Self { return NeoLookup::default() }
    }

    impl<'p, PARAMS> Client<PARAMS> for NeoLookup
    where
        PARAMS: Params,
    {
        const BASE_URL: &'static str = "https://api.nasa.gov/neo/rest/v1/neo";
        type Response = serde_json::Value;

        fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>>
        where
            PARAMS: Params,
        {
            let base_url = <NeoLookup as Client<PARAMS>>::BASE_URL;
            let url_with_params = format!("{}?{}", base_url, params.into());
            let url_with_key = crate::prelude::keys::include(&url_with_params)?;
            let response = ureq::get(&url_with_key).call()?;
            let json = serde_json::json!(response.into_string()?);
            Ok(json)
        }
    }
}
