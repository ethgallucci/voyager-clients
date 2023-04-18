use crate::core::Client;
use std::error::Error;

#[doc = "EPIC API client"]
#[derive(Debug, Clone)]
pub struct EPIC<'p> {
    mode: super::epic_parameters::EPICParams<'p>,
}

impl<'p> Default for EPIC<'p> {
    fn default() -> Self {
        EPIC { mode: super::epic_parameters::EPICParams::NaturalAll }
    }
}

#[allow(missing_docs)]
impl<'p> EPIC<'p> {
    pub fn new() -> Self {
        EPIC::default()
    }
    pub fn mode(&self) -> &super::epic_parameters::EPICParams<'p> {
        return &self.mode
    }
}

impl<'p, P> Client<P> for EPIC<'p>
where
    P: crate::core::Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/EPIC/api";
    type Response = serde_json::Value;

    fn get(&self, params: P) -> Result<Self::Response, Box<dyn Error>> {
        use super::epic_parameters::EPICParams as EpicParams;

        let base_url = <EPIC as Client<P>>::BASE_URL;
        let mode = self.mode();
        let query = match mode {
            EpicParams::NaturalAll => {
                let query_str = format!("{}/natural/all?", base_url);
                crate::prelude::keys::include(&query_str)?
            },
            EpicParams::NaturalAvailable => {
                let query_str = format!("{}/natural?", base_url);
                crate::prelude::keys::include(&query_str)?
            },
            EpicParams::NaturalDate(date) => {
                let query_str = format!("{}/natural/date/{}?", base_url, date);
                crate::prelude::keys::include(&query_str)?
            },
            // Default to NaturalAll
            _ => {
                let query_str = format!("{}/natural/all?", base_url);
                crate::prelude::keys::include(&query_str)?
            }
        };
        let response = ureq::get(&query).call()?.into_string()?;
        let json: serde_json::Value = serde_json::from_str(&response)?;
        return Ok(json)
    }
}

#[cfg(test)]
mod epic_tests {
    use super::*;
    use crate::clients::epic::epic_parameters::EPICParams as EpicParams;

    #[test]
    fn test_epic_client() {
        let epic = EPIC::default();
        let params = EpicParams::NaturalAvailable;
        let response = epic.get(params);
        match response {
            Ok(json) => println!("{:#?}", json),
            Err(e) => {
                println!("{:#?}", e);
                panic!()
            }
        }
    }
}
