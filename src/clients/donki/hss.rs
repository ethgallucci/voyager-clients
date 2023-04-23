use crate::prelude::{Params, SubClient};

/// Parameters for the High Speed Stream (HSS) Endpoint
pub type HSSParams<'p> = crate::prelude::params::DefaultParams<'p>;

/// The High Speed Stream (HSS) SubClient
#[derive(Debug, Clone)]
pub struct HSS {}

impl Default for HSS {
    fn default() -> Self {
        Self {}
    }
}

impl HSS {
    /// Create a new HSS SubClient
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'p, PARAMS> SubClient<PARAMS> for HSS
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/HSS";
}

#[cfg(test)]
mod hss_test {
    use super::{HSSParams as HssPara, HSS as Hss};
    use crate::prelude::__x::*;

    #[test]
    fn test_hss() -> Result<(), Box<dyn Error>> {
        let (hss, hsspara) = (Hss::default(), HssPara::default());
        let nerva: Nerva<Hss, HssPara> = Nerva::new(hss, hsspara);
        let resp = nerva.get()?;
        println!("{:?}", resp);
        return Ok(());
    }
}
