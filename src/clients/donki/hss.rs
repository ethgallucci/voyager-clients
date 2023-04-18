use crate::prelude::{Client, Params};
use std::error::Error;

/// Parameters for the High Speed Stream (HSS) Endpoint
pub type HSSParams<'p> = crate::prelude::params::DefaultParams<'p>;

/// The High Speed Stream (HSS) Client
#[derive(Debug, Clone)]
pub struct HSS {}

impl Default for HSS {
    fn default() -> Self {
        Self {}
    }
}

impl HSS {
    /// Create a new HSS Client
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'p, PARAMS> Client<PARAMS> for HSS
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/HSS";
    type Response = serde_json::Value;

    fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>> {
        let base_url = <HSS as Client<PARAMS>>::BASE_URL;
        let query = &crate::core::util::concat_query(base_url, params.into().as_str())?;
        let res = ureq::get(&query).call()?.into_string()?;
        let json = serde_json::from_str(&res)?;
        return Ok(json);
    }
}

#[cfg(test)]
mod hss_test {
    use super::*;

    #[test]
    fn test_hss() -> Result<(), Box<dyn Error>> {
        let hss = HSS::new();
        let params = HSSParams::default();
        let res = hss.get(params);
        assert!(res.is_ok());
        println!("{:#?}", res?);
        return Ok(());
    }
}
