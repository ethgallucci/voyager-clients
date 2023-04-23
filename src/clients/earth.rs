use crate::prelude::{Params, SubClient};

/// Params for the Earth API
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EarthParams<'p> {
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

impl<'p> Default for EarthParams<'p> {
    fn default() -> Self {
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
impl<'p> EarthParams<'p> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lat(mut self, lat: f64) -> Self {
        self.lat = lat;
        return self;
    }

    pub fn lon(mut self, lon: f64) -> Self {
        self.lon = lon;
        return self;
    }

    pub fn dim(mut self, dim: f64) -> Self {
        self.dim = Some(dim);
        return self;
    }

    pub fn date(mut self, date: &'p str) -> Self {
        self.date = Some(date);
        return self;
    }

    pub fn cloud_score(mut self, cloud_score: bool) -> Self {
        self.cloud_score = Some(cloud_score);
        return self;
    }
}

impl<'p> Into<String> for EarthParams<'p> {
    fn into(self) -> String {
        let mut params = String::new();
        params.push_str(&format!("lat={}", self.lat));
        params.push_str(&format!("&lon={}", self.lon));

        if let Some(dim) = self.dim {
            params.push_str(&format!("&dim={}", dim));
        }
        if let Some(date) = self.date {
            params.push_str(&format!("&date={}", date));
        }
        if let Some(cloud_score) = self.cloud_score {
            params.push_str(&format!("&cloud_score={}", cloud_score));
        }

        return params;
    }
}

impl<'p> Params for EarthParams<'p> {}

/// Earth API client
#[derive(Clone, Debug)]
pub struct Earth {}

impl Default for Earth {
    fn default() -> Self {
        Self {}
    }
}

#[allow(missing_docs)]
impl Earth {
    pub fn new() -> Self {
        return Self::default();
    }
}

impl<'p, PARA> SubClient<PARA> for Earth
where
    PARA: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/planetary/earth/imagery";
}

#[cfg(test)]
mod earth_tests {
    use super::{Earth, EarthParams as EarthPara};
    use crate::prelude::__x::*;

    #[test]
    fn test_earth() {
        let (e, epara) = (Earth::default(), EarthPara::default());
        let nerva: Nerva<Earth, EarthPara> = Nerva::new(e, epara);
    }
}
