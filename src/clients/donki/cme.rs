use crate::prelude::{Client, Params};
use std::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMEParams<'p>
{
    StartDate(&'p str),
    EndDate(&'p str),
}

impl<'p> Into<String> for CMEParams<'p>
{
    fn into(self) -> String
    {
        match self
        {
            CMEParams::StartDate(date) => format!("startDate={}", date),
            CMEParams::EndDate(date) => format!("endDate={}", date),
        }
    }
}

impl<'p> Default for CMEParams<'p>
{
    fn default() -> Self { CMEParams::StartDate("2023-01-01") }
}

impl<'p> Params for CMEParams<'p> {}

#[derive(Clone, Debug)]
pub struct CME {}

impl Default for CME
{
    fn default() -> Self { return Self {} }
}

impl CME
{
    pub fn new() -> Self { return Self::default() }
}

impl<'p, PARAMS> Client<PARAMS> for CME
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/CME";
    type Response = serde_json::Value;

    fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>>
    {
        let burl = <CME as Client<PARAMS>>::BASE_URL;
        let url_with_params = format!("{}?{}", burl, params.into());
        let url_with_key = crate::prelude::keys::include(&url_with_params)?;
        let response = ureq::get(&url_with_key).call()?;
        let json = serde_json::json!(response.into_string()?);
        return Ok(json);
    }
}

#[cfg(test)]
mod cme_test
{
    use super::*;

    #[test]
    fn test_cme()
    {
        let cme = CME::default();
        let response = cme.get(CMEParams::default());
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
