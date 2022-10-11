#[doc = "A common interface for all NASA Open APIs"]
pub trait OpenApiClient {
    /// Must impl From<x> for String for all x of this type Params
    type Params;
    /// A base url endpoint for the API
    const CONNECTION: &'static str;
    /// Query the API with the given parameters
    fn get(&self, params: Self::Params) -> Result<String, anyhow::Error>;
}
