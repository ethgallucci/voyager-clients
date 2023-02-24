use crate::prelude::{Client, Params};
use crate::prelude::params::DefaultParams;
use std::error::Error;

pub type SEPParams<'p> = DefaultParams<'p>;

#[derive(Debug, Clone)]
pub struct SEP {}

impl Default for SEP {
    fn default() -> Self {
        SEP {}
    }
}

impl SEP {
    pub fn new() -> Self {
        SEP::default()
    }
}

impl<'p, PARAMS> Client<PARAMS> for SEP
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/SEP";
    type Response = serde_json::Value;

    fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>> {
        let base_url = <SEP as Client<PARAMS>>::BASE_URL;
        let url_with_params = format!("{}?{}", base_url, params.into());
        let url_with_key = crate::prelude::keys::include(&url_with_params)?;
        let response = ureq::get(&url_with_key).call()?.into_string()?;
        let json: serde_json::Value = serde_json::from_str(&response)?;
        return Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sep() {
        let sep = SEP::new();
        let params = SEPParams::default();
        let response = sep.get(params);
        assert!(response.is_ok());
    }
}
