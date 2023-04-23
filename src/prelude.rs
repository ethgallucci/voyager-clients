pub use crate::core::{Params, SubClient};

/// Util for handling API keys
pub mod keys {
    /// Loads an API key from .env file with key NASA_API_KEY
    pub fn load() -> Result<String, Box<dyn std::error::Error>> {
        dotenv::dotenv()?;
        std::env::var("NASA_API_KEY").map_err(|_| "NASA_API_KEY not found in .env file".into())
    }
    /// Loads an API key from .env file with key NASA_API_KEY and appends it to the query
    pub fn include(query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let key = load()?;
        return Ok(format!("{}&api_key={}", query, key));
    }
}

/// Re-exports client parameters
pub mod params {
    pub use crate::clients::apod::ApodParams as ApodPara;
    pub use crate::clients::donki::cme::CMEParams as CmePara;
    pub use crate::clients::donki::flr::FLRParams as FlrPara;
    pub use crate::clients::donki::gst::GSTParams as GstPara;
    pub use crate::clients::donki::hss::HSSParams as HssPara;
    pub use crate::clients::donki::ips::IPSParams as IpsPara;
    pub use crate::clients::donki::mpc::MPCParams as MpcPara;
    pub use crate::clients::donki::rbe::RBEParams as RbePara;
    pub use crate::clients::donki::sep::SEPParams as SepPara;
    pub use crate::clients::earth::EarthParams as EarthPara;
    pub use crate::clients::neo::{
        feed::NeoFeedParams as NeoFPara, lookup::NeoLookupParams as NeoLPara,
    };

    /// Default parameters for any `SubClient`
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum DefaultParams<'p> {
        #[doc = "Start date for the query"]
        StartDate(&'p str),
        #[doc = "End date for the query"]
        EndDate(&'p str),
    }

    impl<'p> Default for DefaultParams<'p> {
        fn default() -> Self {
            Self::StartDate("2022-01-01")
        }
    }

    impl<'p> Into<String> for DefaultParams<'p> {
        fn into(self) -> String {
            match self {
                Self::StartDate(date) => format!("startDate={}", date),
                Self::EndDate(date) => format!("endDate={}", date),
            }
        }
    }

    impl<'p> crate::core::Params for DefaultParams<'p> {}
}

/// Re-exports common core types and interfaces
pub mod __x {
    pub use super::params::*;
    pub use crate::core::{Client, NervaClient as Nerva};
    pub use crate::prelude::{Params, SubClient};
    pub use std::error::Error;
}

#[cfg(test)]
mod test {
    use super::keys;

    #[test]
    fn load_key() {
        // if .env does not exist, pass this test
        if dotenv::dotenv().is_err() {
            return;
        }

        let key = keys::load();
        assert!(key.is_ok());
        println!("Key: {}", key.unwrap());
    }
}
