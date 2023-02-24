use crate::prelude::{Client, Params};
use std::error::Error;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GSTParams<'p>
{
    StartDate(&'p str),
    EndDate(&'p str),
    Empty,
}

impl Default for GSTParams<'_>
{
    fn default() -> Self { GSTParams::Empty }
}

impl<'p> Into<String> for GSTParams<'p>
{
    fn into(self) -> String
    {
        match self
        {
            GSTParams::StartDate(date) => format!("startDate={}", date),
            GSTParams::EndDate(date) => format!("endDate={}", date),
            GSTParams::Empty => String::new(),
        }
    }
}

impl<'p> Params for GSTParams<'p> {}

#[derive(Debug, Clone)]
pub struct GST {}

impl Default for GST
{
    fn default() -> Self { GST {} }
}

impl GST
{
    pub fn new() -> GST { GST::default() }
}

impl<'p, PARAMS> Client<PARAMS> for GST
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/GST";
    type Response = serde_json::Value;

    fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>>
    {
        let base_url = <GST as Client<PARAMS>>::BASE_URL;
        let url_with_params = format!("{}?{}", base_url, params.into());
        let url_with_key = crate::prelude::keys::include(&url_with_params)?;
        if cfg!(test)
        {
            println!("URL: {}", url_with_key)
        };
        let response = ureq::get(&url_with_key).call()?.into_string()?;
        let json_str: serde_json::Value = serde_json::from_str(&response)?;
        return Ok(json_str);
    }
}

#[cfg(test)]
mod gst_test
{
    use super::*;

    #[test]
    fn test_gst()
    {
        let gst = GST::new();
        let params = GSTParams::default();
        let response = gst.get(params);
        match response
        {
            Ok(json) => println!("{:#?}", json),
            Err(e) =>
            {
                println!("{}", e);
                panic!()
            }
        }
    }
}
