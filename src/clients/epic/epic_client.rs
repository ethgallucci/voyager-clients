use super::epic_parameters::EPICParams as EpicParams;
use crate::core::Client;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct EPIC {}

impl Default for EPIC
{
    fn default() -> Self { EPIC {} }
}

impl EPIC
{
    pub fn new() -> Self { EPIC::default() }
}

impl<'p, P> Client<P> for EPIC
where
    P: crate::core::Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/EPIC/api";
    type Response = serde_json::Value;

    fn get(&self, params: P) -> Result<Self::Response, Box<dyn Error>>
    {
        let base_url = <EPIC as Client<P>>::BASE_URL;
        let url_with_params = format!("{}/{}", base_url, params.into());
        let url_with_key = crate::prelude::keys::include(&url_with_params)?;
        let response = ureq::get(&url_with_key).call()?;
        let json = serde_json::json!(response.into_string()?);
        return Ok(json);
    }
}

#[cfg(test)]
mod epic_tests
{
    use super::*;

    #[test]
    fn test_epic_client()
    {
        let epic = EPIC::default();
        let params = EpicParams::NaturalAll;
        let response = epic.get(params);
        match response
        {
            Ok(json) => println!("{:#?}", json),
            Err(e) =>
            {
                println!("{:#?}", e);
                panic!()
            }
        }
    }
}
