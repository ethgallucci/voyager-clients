use crate::core::{Params, SubClient};
use std::error::Error;

/// Retrieve a list of Asteroids based on their closest approach date to Earth
pub mod feed {
    use super::{Params, SubClient};

    /// Params for the Neo <feed> API
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum NeoFeedParams<'p> {
        #[doc = "Start date of the interval"]
        StartDate(&'p str),
        #[doc = "End date of the interval"]
        EndDate(&'p str),
    }

    impl<'p> Into<String> for NeoFeedParams<'p> {
        fn into(self) -> String {
            match self {
                NeoFeedParams::StartDate(date) => format!("start_date={}", date),
                NeoFeedParams::EndDate(date) => format!("end_date={}", date),
            }
        }
    }

    impl<'p> Default for NeoFeedParams<'p> {
        fn default() -> Self {
            return NeoFeedParams::StartDate("2023-01-01");
        }
    }

    impl<'p> Params for NeoFeedParams<'p> {}

    /// Neo <feed> client
    #[derive(Clone, Debug)]
    pub struct NeoFeed {}

    #[allow(missing_docs)]
    impl NeoFeed {
        pub fn new() -> Self {
            return NeoFeed::default();
        }
    }

    impl Default for NeoFeed {
        fn default() -> Self {
            return NeoFeed {};
        }
    }

    impl<'p, PARAMS> SubClient<PARAMS> for NeoFeed
    where
        PARAMS: Params,
    {
        const BASE_URL: &'static str = "https://api.nasa.gov/neo/rest/v1/feed";
    }

    #[cfg(test)]
    mod tests {
        use super::{NeoFeed as Neof, NeoFeedParams as NeofPara};
        use crate::core::*;

        #[test]
        fn test_neo() {
            let (neof, neofpara) = (Neof::default(), NeofPara::default());
            let nerva: NervaClient<Neof, NeofPara> = NervaClient::new(neof, neofpara);
            let resp = nerva.get().unwrap();
            println!("{:?}", resp);
        }
    }
}

/// Lookup a specific Asteroid based on its NASA JPL small body (SPK-ID) ID
pub mod lookup {
    use super::{Params, SubClient};

    /// Params for the Neo <lookup> API
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum NeoLookupParams<'p> {
        #[doc = "Asteroid SPK-ID correlates to the NASA JPL small body"]
        AsteroidID(&'p str),
    }

    impl<'p> Into<String> for NeoLookupParams<'p> {
        fn into(self) -> String {
            match self {
                NeoLookupParams::AsteroidID(id) => format!("asteroid_id={}", id),
            }
        }
    }

    impl<'p> Default for NeoLookupParams<'p> {
        fn default() -> Self {
            return NeoLookupParams::AsteroidID("2021277");
        }
    }

    impl<'p> Params for NeoLookupParams<'p> {}

    /// Neo <lookup> client
    #[derive(Clone, Debug)]
    pub struct NeoLookup {}

    impl Default for NeoLookup {
        fn default() -> Self {
            return NeoLookup {};
        }
    }

    #[allow(missing_docs)]
    impl NeoLookup {
        pub fn new() -> Self {
            return NeoLookup::default();
        }
    }

    impl<'p, PARAMS> SubClient<PARAMS> for NeoLookup
    where
        PARAMS: Params,
    {
        const BASE_URL: &'static str = "https://api.nasa.gov/neo/rest/v1/neo";
    }
}
