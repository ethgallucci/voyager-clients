#[doc = "A common interface for all NASA Open APIs"]
pub trait OpenApiClient {
    /// Must impl From<x> for String for all x of this type Params
    type Params;
    /// A base url endpoint for the API
    const CONNECTION: &'static str;
    /// Query the API with the given parameters
    fn get(&self, params: Self::Params) -> Result<String, anyhow::Error>;
}

#[doc = "Common params"]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum CommonParams {
    StartDate(String),
    EndDate(String),
}

impl From<CommonParams> for String {
    fn from(params: CommonParams) -> Self {
        #[rustfmt::skip]
        match params {
            CommonParams::StartDate(date) => { format!("startDate={}", date) },
            CommonParams::EndDate(date) => { format!("endDate={}", date) },
        }
    }
}

#[doc = "A common method for all NASA Open APIs, utilized in the `get` trait method"]
pub fn query(url: &str) -> Result<String, anyhow::Error> {
    let response = ureq::get(url).call();
    #[rustfmt::skip]
    match response {
        Ok(r) => { return Ok(r.into_string()?) },
        Err(e) => { return Err(anyhow::anyhow!(e)) },
    }
}
