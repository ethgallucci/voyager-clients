use crate::core::{Params, SubClient};

/// Params for the APOD API
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ApodParams<'p> {
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

impl<'p> Into<String> for ApodParams<'p> {
    fn into(self) -> String {
        match self {
            ApodParams::Date(date) => format!("date={}", date),
            ApodParams::StartDate(date) => format!("start_date={}", date),
            ApodParams::EndDate(date) => format!("end_date={}", date),
            ApodParams::Count(count) => format!("count={}", count),
            ApodParams::Thumbs(thumbs) => format!("thumbs={}", thumbs),
        }
    }
}

impl<'p> Default for ApodParams<'p> {
    fn default() -> Self {
        let date = "2020-01-01";
        return ApodParams::Date(date);
    }
}

impl<'p> Params for ApodParams<'p> {}

/// APOD API client
#[derive(Clone, Debug)]
pub struct Apod {}

impl Default for Apod {
    fn default() -> Self {
        Self {}
    }
}

impl<'p, PARAMS> SubClient<PARAMS> for Apod
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/planetary/apod";
}
