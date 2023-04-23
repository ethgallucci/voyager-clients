use crate::core::SubClient;

/// EPIC API client
#[derive(Debug, Clone)]
pub struct EPIC<'p> {
    mode: super::epic_parameters::EPICParams<'p>,
}

impl<'p> Default for EPIC<'p> {
    fn default() -> Self {
        EPIC {
            mode: super::epic_parameters::EPICParams::NaturalAll,
        }
    }
}

#[allow(missing_docs)]
impl<'p> EPIC<'p> {
    pub fn new() -> Self {
        EPIC::default()
    }
    pub fn mode(&self) -> &super::epic_parameters::EPICParams<'p> {
        return &self.mode;
    }
    /// EPIC has a different URL structure than the other APIs
    pub fn point<P: crate::core::Params>(&self, _params: P) -> String {
        use super::epic_parameters::EPICParams as EpicParams;

        let mut url = String::from(<EPIC as SubClient<P>>::BASE_URL);
        let mode = self.mode();
        let _ = match mode {
            EpicParams::NaturalAll => {
                let query_str = String::from("/natural/all?");
                url.push_str(&query_str);
            }
            EpicParams::NaturalAvailable => {
                let query_str = String::from("/natural?");
                url.push_str(&query_str);
            }
            EpicParams::NaturalDate(date) => {
                let query_str = format!("/natural/date/{}?", date);
                url.push_str(&query_str);
            }
            // Default to NaturalAll
            _ => {
                let query_str = String::from("/natural/all?");
                url.push_str(&query_str);
            }
        };
        return url;
    }
}

impl<'p, P> SubClient<P> for EPIC<'p>
where
    P: crate::core::Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/EPIC/api";
}

#[cfg(test)]
mod epic_tests {
    use super::EPIC as Epic;
    use crate::clients::epic::epic_parameters::EPICParams as EpicPara;

    #[test]
    fn test_epic_client() {
        let (epic, epicpara) = (Epic::default(), EpicPara::default());
    }
}
