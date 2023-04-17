use crate::prelude::{Client, Params};
use std::error::Error;

type MPCParams<'p> = crate::prelude::params::DefaultParams<'p>;

/// The `MPC` client
#[derive(Debug, Clone)]
pub struct MPC {}

impl Default for MPC {
    fn default() -> Self {
        Self {}
    }
}

impl MPC {
    /// Create a new `MPC` client
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'p, PARAMS> Client<PARAMS> for MPC
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/MPC";
    type Response = serde_json::Value;

    fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>> {
        let base_url = <MPC as Client<PARAMS>>::BASE_URL;
        let query = crate::core::util::concat_query(base_url, params.into().as_str())?;
        let response = ureq::get(&query).call()?.into_string()?;
        let json: serde_json::Value = serde_json::from_str(&response)?;
        return Ok(json);
    }
}

#[cfg(test)]
mod mpc_tests {
    use super::*;

    #[test]
    fn test_mpc() {
        let mpc = MPC::new();
        let params = MPCParams::default();
        let response = mpc.get(params);
        assert!(response.is_ok());
        println!("{:#?}", response.unwrap());
    }
}
