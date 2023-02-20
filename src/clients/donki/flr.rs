use crate::prelude::params::DefaultParams;
use crate::prelude::{Client, Params};
use std::error::Error;

pub type FLRParams<'p> = DefaultParams<'p>;

#[derive(Debug, Clone)]
pub struct FLR {}

impl Default for FLR
{
    fn default() -> Self { Self {} }
}

impl FLR
{
    pub fn new() -> Self { Self::default() }
}

impl<'p, PARAMS> Client<PARAMS> for FLR
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/FLR";
    type Response = serde_json::Value;

    fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>>
    {
        let base_url = <FLR as Client<PARAMS>>::BASE_URL;
        let url_with_params = format!("{}?{}", base_url, params.into());
        let url_with_key = crate::prelude::keys::include(&url_with_params)?;
        let response = ureq::get(&url_with_key).call()?;
        let json = serde_json::json!(response.into_string()?);
        return Ok(json);
    }
}

#[cfg(test)]
mod flr_tests
{
    use super::*;

    #[test]
    fn flr_test() -> Result<(), Box<dyn Error>>
    {
        let flr = FLR::new();
        let params = FLRParams::default();
        let response = flr.get(params)?;
        println!("{:#?}", response);
        Ok(())
    }
}
