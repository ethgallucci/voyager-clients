use crate::prelude::params::DefaultParams;
use crate::prelude::{Client, Params};
use std::error::Error;

pub type RBEParams<'p> = DefaultParams<'p>;

#[derive(Debug, Clone)]
pub struct RBE {}

impl Default for RBE
{
    fn default() -> Self { Self {} }
}

impl RBE
{
    pub fn new() -> Self { Self::default() }
}

impl<'p, DEP> Client<DEP> for RBE
where
    DEP: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/RBE";
    type Response = serde_json::Value;

    fn get(&self, params: DEP) -> Result<Self::Response, Box<dyn Error>>
    {
        let base_url = <RBE as Client<DEP>>::BASE_URL;
        let url_with_params = format!("{}?{}", base_url, params.into());
        let url_with_key = crate::prelude::keys::include(&url_with_params)?;
        let response = ureq::get(&url_with_key).call()?.into_string()?;
        let json: serde_json::Value = serde_json::from_str(&response)?;
        Ok(json)
    }
}

#[cfg(test)]
mod rbe_tests
{
    use super::*;

    #[test]
    fn test_rbe()
    {
        pretty_env_logger::try_init().ok();
        let rbe = RBE::new();
        let params = RBEParams::default();
        log::info!("params: {:?}", params);
        let response = rbe.get(params);
        log::debug!("response: {:?}", response);
        assert!(response.is_ok());
    }
}
