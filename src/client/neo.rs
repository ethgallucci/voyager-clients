use crate::prelude::*;

#[doc = "Mode for the NEO client: feed, browse, or lookup"]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NeoMode {
    #[doc = "Feed mode"]
    FEED,
    #[doc = "Browse mode"]
    BROWSE,
    #[doc = "Lookup mode"]
    LOOKUP,
}

#[doc = "Client for the NEO API"]
#[rustfmt::skip]
pub struct NeoClient { 
    mode: NeoMode,
}

impl Default for NeoClient {
    fn default() -> Self {
        Self {
            mode: NeoMode::FEED,
        }
    }
}

#[doc = "Parameters for the NEO API"]
#[allow(missing_docs)]
pub enum NeoParams {
    StartDate(String),
    EndDate(String),
    DateRange { start: String, end: String },
    Browse,
    AsteroidId(String),
}

impl Default for NeoParams {
    fn default() -> Self {
        let two_weeks_ago = chrono::Utc::today()
            .checked_sub_signed(chrono::Duration::weeks(2))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
        let today = chrono::Utc::today().format("%Y-%m-%d").to_string();
        return NeoParams::DateRange {
            start: two_weeks_ago,
            end: today,
        };
    }
}

impl From<NeoParams> for String {
    fn from(params: NeoParams) -> Self {
        #[rustfmt::skip]
        match params {
            NeoParams::StartDate(date) => { format!("start_date={}", date) },
            NeoParams::EndDate(date) => { format!("end_date={}", date) },
            NeoParams::DateRange { start, end } => { format!("start_date={}&end_date={}", start, end) },
            NeoParams::Browse => { format!("browse") },
            NeoParams::AsteroidId(id) => { format!("{}", id) },
        }
    }
}

impl NeoClient {
    #[allow(missing_docs)]
    pub fn new(mode: NeoMode) -> Self {
        Self { mode }
    }
}

impl OpenApiClient for NeoClient {
    type Params = NeoParams;
    const CONNECTION: &'static str = "https://api.nasa.gov/neo/rest/v1/";

    fn get(&self, params: Self::Params) -> Result<String, anyhow::Error> {
        let mut url = match self.mode {
            NeoMode::FEED => String::from(Self::CONNECTION) + "feed?",
            NeoMode::BROWSE => String::from(Self::CONNECTION) + "browse/",
            NeoMode::LOOKUP => String::from(Self::CONNECTION) + "neo/",
        };
        url.push_str(&format!("{}", String::from(params)));
        url.push_str(&format!("&api_key={}", crate::key::load("API_KEY")));

        return query(&url);
    }
}
