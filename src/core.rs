//! Defines the core types of the library.

use reqwest::Client;

pub trait Params
where
    Self: Into<String> + PartialEq + Clone + Copy + Default,
{
}

#[cfg(any(feature = "response_ext"))]
pub trait ResponseExt
where
    Self: serde::de::DeserializeOwned + core::fmt::Debug,
{
    fn into_result(self) -> Result<Self, reqwest::Error>;
}

pub trait OpenApi<P>
where
    P: Params,
{
    type Params = P;
    type Response: core::fmt::Debug;

    fn get(&self, params: Self::Params) -> Self::Response;
    fn get_raw(&self, params: Self::Params) -> Self::Response;
    fn get_raw_with_headers(
        &self,
        params: Self::Params,
        headers: reqwest::header::HeaderMap,
    ) -> Self::Response;
}

pub struct ClientBuilder<O, P>
where
    O: OpenApi<P>,
    P: Params,
{
    base_url: &'static str,
    client: reqwest::Client,
    _marker: std::marker::PhantomData<(O, P)>,
}

impl<O, P> ClientBuilder<O, P>
where
    O: OpenApi<P>,
    P: Params,
{
    pub fn new(base_url: &'static str) -> Self
    {
        Self {
            base_url,
            client: Client::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn client_mut(&mut self, client: reqwest::Client) -> &mut reqwest::Client
    {
        return &mut self.client;
    }
}
