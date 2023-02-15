use core::fmt::Debug as DebugImpl;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub trait Params
where
    Self: Into<String> + Clone + Copy + DebugImpl + PartialEq + Default,
{
}

pub trait Client<P>
where
    P: Params,
    Self: Default,
{
    const BASE_URL: &'static str;
    type Response: for<'de> Deserialize<'de> + Serialize + Clone + DebugImpl + PartialEq;

    fn get(&self, params: P) -> Result<Self::Response, Box<dyn Error>>;
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::clients::apod::{Apod, ApodParams};

    #[test]
    fn test_apod()
    {
        let apod = Apod::default();
        let res = apod.get(ApodParams::default());
        match res
        {
            Ok(json) => println!("{:#?}", json),
            Err(e) => println!("{:#?}", e),
        }
    }
}
