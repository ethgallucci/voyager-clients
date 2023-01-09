use crate::prelude::*;

#[doc = "Client / Params for the Coronal Mass Ejection API"]
pub mod cme {
    use super::*;

    #[doc = "Client for the Coronal Mass Ejection API"]
    pub struct CoronalMassEjectionClient {
        analysis: bool,
    }

    impl CoronalMassEjectionClient {
        #[doc = "Create a new client"]
        pub fn new(analysis: bool) -> Self {
            Self { analysis }
        }
    }

    impl Default for CoronalMassEjectionClient {
        fn default() -> Self {
            Self { analysis: false }
        }
    }

    #[allow(non_camel_case_types, missing_docs)]
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum CatalogChoices {
        ALL,
        SWRC_CATALOG,
        JANG_ET_AL_CATALOG,
    }

    impl From<CatalogChoices> for String {
        fn from(choice: CatalogChoices) -> Self {
            match choice {
                CatalogChoices::ALL => "ALL".to_string(),
                CatalogChoices::SWRC_CATALOG => "SWRC_CATALOG".to_string(),
                CatalogChoices::JANG_ET_AL_CATALOG => "JANG_ET_AL_CATALOG".to_string(),
            }
        }
    }

    #[doc = "Parameters for the Coronal Mass Ejection API"]
    #[allow(missing_docs)]
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum CMEParams {
        /// Available in both modes
        StartDate(String),
        /// Available in both modes
        EndDate(String),
        /// Only available in the analysis mode
        #[doc = "Default is set to: True"]
        MostAccurateOnly,
        /// Only available in the analysis mode
        #[doc = "Default is set to: True"]
        CompleteEntryOnly,
        /// Only available in the analysis mode
        #[doc = "Default is set to: 0"]
        Speed(usize),
        /// Only available in the analysis mode
        #[doc = "Default is set to: 0"]
        /// Only available in the analysis mode
        HalfAngle(usize),
        Catalog(CatalogChoices),
    }

    impl From<CMEParams> for String {
        fn from(params: CMEParams) -> Self {
            #[rustfmt::skip]
            match params {
                CMEParams::StartDate(date) => { format!("startDate={}", date) },
                CMEParams::EndDate(date) => { format!("endDate={}", date) },
                CMEParams::MostAccurateOnly => { format!("mostAccurateOnly=true") },
                CMEParams::CompleteEntryOnly => { format!("completeEntryOnly=true") },
                CMEParams::Speed(speed) => { format!("speed={}", speed) },
                CMEParams::HalfAngle(angle) => { format!("halfAngle={}", angle) },
                CMEParams::Catalog(catalog) => { format!("catalog={}", String::from(catalog)) },
            }
        }
    }

    impl OpenApiClient for CoronalMassEjectionClient {
        type Params = CMEParams;
        const CONNECTION: &'static str = "https://api.nasa.gov/DONKI/CME";

        fn get(&self, params: Self::Params) -> Result<String, anyhow::Error> {
            if !self.analysis {
                match params {
                    CMEParams::StartDate(_) => {}
                    CMEParams::EndDate(_) => {}
                    _ => return Err(anyhow::anyhow!("Invalid parameter for this mode")),
                }
            }

            let mut url = match self.analysis {
                true => String::from(Self::CONNECTION) + "Analysis?",
                false => String::from(Self::CONNECTION) + "?",
            };
            url.push_str(&format!("{}", String::from(params)));
            url.push_str(&format!("&api_key={}", crate::key::load("NASA_API_KEY")));

            return query(&url);
        }
    }
}

#[doc = "Client / Params for the Geomagnetic Storm API"]
pub mod gst {
    use super::*;

    #[doc = "Client for the Geomagnetic Storm API"]
    pub struct GeoMagneticStormClient {}

    impl GeoMagneticStormClient {
        #[doc = "Create a new client"]
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Default for GeoMagneticStormClient {
        fn default() -> Self {
            Self {}
        }
    }

    #[doc = "Parameters for the Geomagnetic Storm API"]
    #[allow(missing_docs)]
    pub type GSTParams = CommonParams;

    impl OpenApiClient for GeoMagneticStormClient {
        type Params = GSTParams;
        const CONNECTION: &'static str = "https://api.nasa.gov/DONKI/GST?";

        fn get(&self, params: Self::Params) -> Result<String, anyhow::Error> {
            let mut url = String::from(Self::CONNECTION);
            url.push_str(&format!("{}", String::from(params)));
            url.push_str(&format!("&api_key={}", crate::key::load("NASA_API_KEY")));

            return query(&url);
        }
    }
}

#[doc = "Client / Params for the Interplanetary Shock API"]
pub mod ips {
    use super::*;

    #[doc = "Client for the Interplanetary Shock API"]
    pub struct InterplanetaryShockClient {}

    impl InterplanetaryShockClient {
        #[doc = "Create a new client"]
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Default for InterplanetaryShockClient {
        fn default() -> Self {
            Self {}
        }
    }

    #[doc = "Location choices for the Interplanetary Shock API"]
    #[allow(missing_docs, non_camel_case_types)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum LocChoice {
        /// The default LocChoice
        ALL,
        EARTH,
        MESSENGER,
        STEREO_A,
        STEREO_B,
    }

    #[doc = "Catalog choices for the Interplanetary Shock API"]
    #[allow(missing_docs, non_camel_case_types)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum CatalogChoice {
        /// The default CatalogChoice
        ALL,
        SWRC_CATALOG,
        WINSLOW_MESSENGER_ICME_CATALOG,
    }

    impl From<LocChoice> for String {
        fn from(choice: LocChoice) -> Self {
            #[rustfmt::skip]
            match choice {
                LocChoice::ALL => { "ALL".to_string() },
                LocChoice::EARTH => { "EARTH".to_string() },
                LocChoice::MESSENGER => { "MESSENGER".to_string() },
                LocChoice::STEREO_A => { "STEREO_A".to_string() },
                LocChoice::STEREO_B => { "STEREO_B".to_string() },
            }
        }
    }

    impl From<CatalogChoice> for String {
        fn from(choice: CatalogChoice) -> Self {
            #[rustfmt::skip]
            match choice {
                CatalogChoice::ALL => { "ALL".to_string() },
                CatalogChoice::SWRC_CATALOG => { "SWRC_CATALOG".to_string() },
                CatalogChoice::WINSLOW_MESSENGER_ICME_CATALOG => { "WINSLOW_MESSENGER_ICME_CATALOG".to_string() },
            }
        }
    }

    #[doc = "Parameters for the Interplanetary Shock API"]
    #[allow(missing_docs)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum IPSParams {
        Location(LocChoice),
        Catalog(CatalogChoice),
        StartDate(String),
        EndDate(String),
    }

    impl From<IPSParams> for String {
        fn from(params: IPSParams) -> Self {
            #[rustfmt::skip]
            match params {
                IPSParams::Location(loc) => { format!("location={}", String::from(loc)) },
                IPSParams::Catalog(catalog) => { format!("catalog={}", String::from(catalog)) },
                IPSParams::StartDate(date) => { format!("startDate={}", date) },
                IPSParams::EndDate(date) => { format!("endDate={}", date) },
            }
        }
    }

    impl OpenApiClient for InterplanetaryShockClient {
        type Params = IPSParams;
        const CONNECTION: &'static str = "https://api.nasa.gov/DONKI/IPS?";

        fn get(&self, params: Self::Params) -> Result<String, anyhow::Error> {
            let mut url = String::from(Self::CONNECTION);
            url.push_str(&format!("{}", String::from(params)));
            url.push_str(&format!("&api_key={}", crate::key::load("NASA_API_KEY")));

            return query(&url);
        }
    }
}
