use crate::prelude::params::DefaultParams;
use crate::prelude::{Params, SubClient};

/// Params for the CME API
pub type CMEParams<'p> = DefaultParams<'p>;

/// CME API client
#[derive(Clone, Debug)]
pub struct CME {}

impl Default for CME {
    fn default() -> Self {
        return Self {};
    }
}

#[allow(missing_docs)]
impl CME {
    pub fn new() -> Self {
        return Self::default();
    }
}

impl<'p, PARAMS> SubClient<PARAMS> for CME
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/CME";
}

#[cfg(test)]
mod cme_test {
    use super::{CMEParams as CmePara, CME as Cme};
    use crate::prelude::__x::*;

    #[test]
    fn test_cme() {
        let (cme, cmepara) = (Cme::default(), CmePara::default());
        let nerva: Nerva<Cme, CmePara> = Nerva::new(cme, cmepara);
        let resp = nerva.get().unwrap();
        println!("{:?}", resp);
    }
}
