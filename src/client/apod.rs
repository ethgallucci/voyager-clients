use crate::prelude::*;

#[doc = "Implementation of the Picture of the Day API"]
    #[rustfmt::skip]
    pub struct ApodClient { }

#[doc = "Parameters for the APOD API"]
#[allow(missing_docs)]
#[derive(Debug, PartialEq)]
pub enum ApodParams {
    // '?date=YYYY-MM-DD'
    Date(String),
    // ?start_date=YYYY-MM-DD
    StartDate(String),
    // ?end_date=YYYY-MM-DD
    EndDate(String),
}

/// Default parameters for the APOD API are today's date
impl Default for ApodParams {
    fn default() -> Self {
        let today = chrono::Utc::today().format("%Y-%m-%d").to_string();
        return ApodParams::Date(today);
    }
}

/// Must impl From<x> for String for all x of this type Params
impl From<ApodParams> for String {
    fn from(params: ApodParams) -> Self {
        #[rustfmt::skip]
            match params {
                ApodParams::Date(date) => { format!("date={}", date) },
                ApodParams::StartDate(date) => { format!("start_date={}", date) },
                ApodParams::EndDate(date) => { format!("end_date={}", date) },
            }
    }
}

impl OpenApiClient for ApodClient {
    // API params
    type Params = ApodParams;
    // Endpoint
    const CONNECTION: &'static str = "https://api.nasa.gov/planetary/apod?";

    // Query w/ params
    fn get(&self, params: Self::Params) -> Result<String, anyhow::Error> {
        // Push our params and api key to the endpoint string
        let mut url = String::from(Self::CONNECTION);
        url.push_str(&format!("{}", String::from(params)));
        url.push_str(&format!("&api_key={}", crate::key::load("API_KEY")));

        // Send the request and return the response
        return query(&url);
    }
}
