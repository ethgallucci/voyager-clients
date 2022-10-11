use crate::OpenApiClient;

#[doc = "Picture of the day"]
pub mod apod {
    use super::*;

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
            let mut base_url = String::from(Self::CONNECTION);
            base_url.push_str(&format!("{}", String::from(params)));
            base_url.push_str(&format!("&api_key={}", crate::key::load("API_KEY")));

            // Send the request and return the response
            let response = ureq::get(&base_url).call();
            #[rustfmt::skip]
            match response {
                Ok(r) => { return Ok(r.into_string()?) },
                Err(e) => { return Err(anyhow::anyhow!(e)) },
            }
        }
    }
}

#[doc = "Near Earth Objects API"]
pub mod neo {
    use super::*;

    #[doc = "Implementation of the Near Earth Objects API"]
    #[rustfmt::skip]
    pub struct NeoClient { }

    #[doc = "Parameters for the NEO API"]
    #[allow(missing_docs)]
    pub enum NeoParams {
        StartDate(String),
        EndDate(String),
    }

    impl From<NeoParams> for String {
        fn from(params: NeoParams) -> Self {
            #[rustfmt::skip]
            match params {
                NeoParams::StartDate(date) => { format!("start_date={}", date) },
                NeoParams::EndDate(date) => { format!("end_date={}", date) },
            }
        }
    }

    impl OpenApiClient for NeoClient {
        type Params = NeoParams;
        const CONNECTION: &'static str = "https://api.nasa.gov/neo/rest/v1/feed?";

        fn get(&self, params: Self::Params) -> Result<String, anyhow::Error> {
            let mut base_url = String::from(Self::CONNECTION);
            base_url.push_str(&format!("{}", String::from(params)));
            base_url.push_str(&format!("&api_key={}", crate::key::load("API_KEY")));

            let response = ureq::get(&base_url).call();
            #[rustfmt::skip]
            match response {
                Ok(r) => { return Ok(r.into_string()?) },
                Err(e) => { return Err(anyhow::anyhow!(e)) },
            }
        }
    }
}
