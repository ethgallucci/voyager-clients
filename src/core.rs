use core::fmt::Debug as DebugImpl;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// API Parameters for all clients
pub trait Params
where
    Self: Into<String> + Clone + Copy + DebugImpl + PartialEq + Default,
{
}

/// A `SubClient` is a wrapper over a specific API endpoint, such as `Apod` or `NeoFeed`
pub trait SubClient<P>
where
    P: Params,
    Self: Default + Clone,
{
    /// The base URL endpoint
    const BASE_URL: &'static str;
}

/// An `Aim` is a pair of a `SubClient` and a `Params` that encapsulates a specific Open API endpoint
#[derive(Debug, Clone, PartialEq)]
pub struct Aim<S, P>(S, P)
where
    S: SubClient<P>,
    P: Params;

impl<S, P> Aim<S, P>
where
    S: SubClient<P>,
    P: Params,
{
    /// Get the `SubClient`
    pub fn subclient(&self) -> &S {
        &self.0
    }

    /// Get the `Params`
    pub fn params(&self) -> &P {
        &self.1
    }

    /// Set the `SubClient`
    pub fn set_subclient(&mut self, subclient: S) {
        self.0 = subclient;
    }

    /// Set the `Params`
    pub fn set_params(&mut self, params: P) {
        self.1 = params;
    }
}

pub(crate) trait IntoAim<S, P>
where
    S: SubClient<P>,
    P: Params,
{
    fn aim(self) -> Aim<S, P>;
}

impl<S, P> From<Aim<S, P>> for NervaClient<S, P>
where
    S: SubClient<P>,
    P: Params,
{
    fn from(aim: Aim<S, P>) -> Self {
        NervaClient::new(aim.0, aim.1)
    }
}

impl<S, P> From<S> for Aim<S, P>
where
    S: SubClient<P>,
    P: Params,
{
    fn from(subclient: S) -> Self {
        Aim(subclient, P::default())
    }
}

impl<S, P> IntoAim<S, P> for (S, P)
where
    S: SubClient<P>,
    P: Params,
{
    fn aim(self) -> Aim<S, P> {
        Aim(self.0, self.1)
    }
}

/// Points a `SubClient` with `Params` to a URL
pub(crate) fn point<S, P>(params: P) -> String
where
    S: crate::core::SubClient<P>,
    P: crate::core::Params,
{
    let mut url = String::from(<S as crate::core::SubClient<P>>::BASE_URL);
    url.push_str("?");
    url.push_str(&params.into());
    return url;
}

/// Points with a slash instead of a question mark
pub(crate) fn point_slash<S, P>(params: P) -> String
where
    S: crate::core::SubClient<P>,
    P: crate::core::Params,
{
    let mut url = String::from(<S as crate::core::SubClient<P>>::BASE_URL);
    url.push_str("/");
    url.push_str(&params.into());
    return url;
}

/// A `Client` interface defines a wrapper over any `SubClient` with some `Params`
pub trait Client<S, P>
where
    S: SubClient<P>,
    P: Params,
{
    /// The response type; yielded from the `get` method
    type Response: for<'de> Deserialize<'de> + Serialize + Clone + DebugImpl + PartialEq =
        serde_json::Value;
    /// The error type; yielded from the `get` method
    type Error = Box<dyn Error>;

    /// Get the response from the API at the `SubClient` endpoint with the `Params`
    fn get(&self) -> Result<Self::Response, Self::Error>;
}

/// The core client type that can handle any `SubClient` with some `Params`
#[derive(Debug, Clone, PartialEq)]
pub struct NervaClient<S, P>
where
    S: SubClient<P>,
    P: Params,
{
    _subclient: S,
    params: P,
}

impl<S, P> Client<S, P> for NervaClient<S, P>
where
    S: SubClient<P>,
    P: Params,
{
    type Response = serde_json::Value;
    type Error = Box<dyn Error>;

    fn get(&self) -> Result<Self::Response, Self::Error> {
        let url = point::<S, P>(self.params);
        let keyed = crate::prelude::keys::include(&url)?;
        let resp = ureq::get(&keyed).call()?.into_string()?;
        let json: serde_json::Value = serde_json::from_str(&resp)?;
        return Ok(json);
    }
}

impl<S, P> NervaClient<S, P>
where
    S: SubClient<P>,
    P: Params,
{
    /// Construct a new `NervaClient` with a `SubClient` and `Params`
    pub fn new(_subclient: S, params: P) -> Self {
        Self { _subclient, params }
    }

    /// Switch to a different pair of `SubClient` and `Params`
    pub fn switch_prim<Sn, Pn>(&self, subnew: Sn, paranew: Pn) -> NervaClient<Sn, Pn>
    where
        Sn: SubClient<Pn>,
        Pn: Params,
    {
        NervaClient::new(subnew, paranew)
    }

    pub fn switch<Sn, Pn>(&self, aim: Aim<Sn, Pn>) -> NervaClient<Sn, Pn>
    where
        Sn: SubClient<Pn>,
        Pn: Params,
    {
        NervaClient::new(aim.0, aim.1)
    }
}

/// Global Util
pub mod util {
    /// Concatenate the base URL with the parameters to form a query string
    /// !!! This includes the API key !!!
    pub fn concat_query(
        base_url: &str,
        params_str: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let with_params = format!("{}?{}", base_url, params_str);
        let with_key = crate::prelude::keys::include(&with_params)?;
        return Ok(with_key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clients::apod::{Apod, ApodParams as ApodPara};
    use crate::clients::neo::feed::{NeoFeed, NeoFeedParams as NeoFeedPara};

    #[test]
    fn test_nerva() {
        // Construct SubClients we want to use
        let apod = Apod::default();
        let neofeed = NeoFeed::default();

        // Construct a CoreClient that handles one specific SubClient at a time
        let nerva = NervaClient::new(neofeed, NeoFeedPara::default());
        let nerva = nerva.switch::<Apod, ApodPara>(apod, ApodPara::default());
        let res = nerva.get().unwrap();
        dbg!(res);
        // or
        // let nerva = NervaClient::<Apod, ApodPara>::new(apod, ApodPara::default());
        // We can switch to a different SubClient
        // or
        // nerva.switch::<NeoFeed, NeoFeedPara>(neofeed, NeoFeedPara::default());
    }
}
