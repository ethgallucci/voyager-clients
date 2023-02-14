use crate::core::{Client as CoreClient, OpenApi};
use serde::{Deserialize, Serialize};

static APOD_BASE_URL: &'static str = "https://api.nasa.gov/planetary/apod";

pub struct ApodClient
{
    base_url: &'static str,
}

impl Default for ApodClient
{
    fn default() -> Self
    {
        Self {
            base_url: APOD_BASE_URL,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum ApodParams<'p>
{
    Date(&'p str),
    Hd(bool),
    ApiKey(&'p str),
}
