use crate::core::{Client, Params};
use std::error::Error;

/// Params for the APOD API
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApodParams<'p>
{
    #[doc = "Date of the APOD image"]
    Date(&'p str),
    #[doc = "Start date of the interval"]
    StartDate(&'p str),
    #[doc = "End date of the interval"]
    EndDate(&'p str),
    #[doc = "Number of images to return"]
    Count(usize),
    #[doc = "Include thumbnail images in the response"]
    Thumbs(bool),
}

impl<'p> Into<String> for ApodParams<'p>
{
    fn into(self) -> String
    {
        match self
        {
            ApodParams::Date(date) => format!("date={}", date),
            ApodParams::StartDate(date) => format!("start_date={}", date),
            ApodParams::EndDate(date) => format!("end_date={}", date),
            ApodParams::Count(count) => format!("count={}", count),
            ApodParams::Thumbs(thumbs) => format!("thumbs={}", thumbs),
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

/// APOD API client
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
        let url_with_key = crate::prelude::keys::include(&url)?;
        let response = ureq::get(&url_with_key).call()?;
        let json = serde_json::json!(response.into_string()?);
        Ok(json)
    }
}
