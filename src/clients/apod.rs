use crate::core::{Client, Params};
use std::error::Error;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApodParams<'p>
{
    Date(&'p str),
    Limit(usize),
}

impl<'p> Into<String> for ApodParams<'p>
{
    fn into(self) -> String
    {
        match self
        {
            ApodParams::Date(date) => format!("date={}", date),
            ApodParams::Limit(limit) => format!("count={}", limit),
        }
    }
}

impl<'p> Default for ApodParams<'p>
{
    fn default() -> Self
    {
        let date = "2020-01-01";
        return ApodParams::Date(date);
    }
}

impl<'p> Params for ApodParams<'p> {}

#[derive(Clone, Debug)]
pub struct Apod {}

impl Default for Apod
{
    fn default() -> Self { Self {} }
}

impl<'p, PARAMS> Client<PARAMS> for Apod
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/planetary/apod";
    type Response = serde_json::Value;

    fn get(&self, params: PARAMS) -> Result<Self::Response, Box<dyn Error>>
    {
        let burl: &'static str = <Apod as Client<PARAMS>>::BASE_URL;
        let url = format!("{}/?{}", burl, params.into());
        let response = ureq::get(&url).call()?;
        let json = serde_json::json!(response.into_string()?);
        Ok(json)
    }
}
