use crate::prelude::{Client, Params};
use std::error::Error;

#[doc = "Parameters for the Earth API"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EarthParams<'p>
{
    #[doc = "Latitude of the location"]
    pub lat: f64,
    #[doc = "Longitude of the location"]
    pub lon: f64,
    #[doc = "Dimension of the image (Optional)"]
    pub dim: Option<f64>,
    #[doc = "Date of the image (Optional)"]
    pub date: Option<&'p str>,
    #[doc = "Include cloud score in the response (Optional)"]
    pub cloud_score: Option<bool>,
}

impl<'p> Default for EarthParams<'p>
{
    fn default() -> Self
    {
        Self {
            lat: 0.0,
            lon: 0.0,
            dim: None,
            date: None,
            cloud_score: None,
        }
    }
}

#[allow(missing_docs)]
impl<'p> EarthParams<'p>
{
    pub fn new() -> Self { Self::default() }

    pub fn lat(mut self, lat: f64) -> Self
    {
        self.lat = lat;
        return self;
    }

    pub fn lon(mut self, lon: f64) -> Self
    {
        self.lon = lon;
        return self;
    }

    pub fn dim(mut self, dim: f64) -> Self
    {
        self.dim = Some(dim);
        return self;
    }

    pub fn date(mut self, date: &'p str) -> Self
    {
        self.date = Some(date);
        return self;
    }

    pub fn cloud_score(mut self, cloud_score: bool) -> Self
    {
        self.cloud_score = Some(cloud_score);
        return self;
    }
}

impl<'p> Into<String> for EarthParams<'p>
{
    fn into(self) -> String
    {
        let mut params = String::new();
        params.push_str(&format!("lat={}", self.lat));
        params.push_str(&format!("&lon={}", self.lon));

        if let Some(dim) = self.dim
        {
            params.push_str(&format!("&dim={}", dim));
        }
        if let Some(date) = self.date
        {
            params.push_str(&format!("&date={}", date));
        }
        if let Some(cloud_score) = self.cloud_score
        {
            params.push_str(&format!("&cloud_score={}", cloud_score));
        }

        return params;
    }
}

impl<'p> Params for EarthParams<'p> {}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Earth {}

impl Default for Earth
{
    fn default() -> Self { Self {} }
}

#[allow(missing_docs)]
impl Earth
{
    pub fn new() -> Self { return Self::default() }
}

impl<'p, PARA> Client<PARA> for Earth
where
    PARA: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/planetary/earth/imagery";
    type Response = serde_json::Value;

    fn get(&self, params: PARA) -> Result<Self::Response, Box<dyn Error>>
    {
        let base_url = <Earth as Client<PARA>>::BASE_URL;
        let url_with_params = format!("{}?{}", base_url, params.into());
        let url_with_key = crate::prelude::keys::include(&url_with_params)?;
        let response = ureq::get(&url_with_key).call()?.into_string()?;
        let json: serde_json::Value = serde_json::from_str(&response)?;
        return Ok(json);
    }
}

#[cfg(test)]
mod earth_tests
{
    use super::*;

    #[test]
    fn test_earth()
    {
        let earth = Earth::default();
        let params = EarthParams::default();
        let response = earth.get(params);
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
