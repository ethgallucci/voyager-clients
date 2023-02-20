use crate::prelude::params::DefaultParams;
use crate::prelude::*;
use std::error::Error;

pub type ISPParams<'p> = DefaultParams<'p>;

#[derive(Debug, Clone)]
pub struct ISP {}

impl Default for ISP
{
    fn default() -> Self { Self {} }
}

impl ISP
{
    pub fn new() -> Self { Self::default() }
}

impl<'p, PARAMS> Client<PARAMS> for ISP
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/IPS";
    type Response = serde_json::Value;

    fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>>
    {
        let base_url = <ISP as Client<PARAMS>>::BASE_URL;
        let url_with_params = format!("{}?{}", base_url, params.into());
        let url_with_key = keys::include(&url_with_params)?;
        let response = ureq::get(&url_with_key).call()?;
        let json = serde_json::json!(response.into_string()?);
        Ok(json)
    }
}

#[cfg(test)]
mod isp_test
{
    use super::*;

    #[test]
    fn test_isp()
    {
        let isp = ISP::new();
        let params = ISPParams::default();
        let response = isp.get(params).unwrap();
        println!("{:#?}", response);
    }
}
