use core::fmt::Debug as DebugImpl;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// API Parameters for all clients
pub trait Params
where
    Self: Into<String> + Clone + Copy + DebugImpl + PartialEq + Default,
{
}

/// Interface behavior for all clients
pub trait Client<P>
where
    P: Params,
    Self: Default,
{
    /// The base URL endpoint
    const BASE_URL: &'static str;
    /// The response type
    type Response: for<'de> Deserialize<'de> + Serialize + Clone + DebugImpl + PartialEq;

    /// Get the response from the API
    fn get(&self, params: P) -> Result<Self::Response, Box<dyn Error>>;
}

/// Global Util
pub mod util {
    /// Concatenate the base URL with the parameters to form a query string
    /// !!! This includes the API key !!!
    pub fn concat_query(
        base_url: &str,
        params_str: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let with_params = format!("{}?{}", base_url, params_str);
        let with_key = crate::prelude::keys::include(&with_params)?;
        return Ok(with_key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::apod::{Apod, ApodParams};

    #[test]
    fn test_apod() {
        let apod = Apod::default();
        let res = apod.get(ApodParams::default());
        match res {
            Ok(json) => println!("{:#?}", json),
            Err(e) => {
                println!("{:#?}", e);
                panic!();
            }
        }
    }
}
