use crate::prelude::params::DefaultParams;
use crate::prelude::{Params, SubClient};

/// Params for the FLR API
pub type FLRParams<'p> = DefaultParams<'p>;

/// FLR API client
#[derive(Debug, Clone)]
pub struct FLR {}

impl Default for FLR {
    fn default() -> Self {
        Self {}
    }
}

#[allow(missing_docs)]
impl FLR {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'p, PARAMS> SubClient<PARAMS> for FLR
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/FLR";
}

#[cfg(test)]
mod flr_tests {
    use super::{FLRParams as FlrPara, FLR as Flr};
    use crate::prelude::__x::*;

    #[test]
    fn flr_test() -> Result<(), Box<dyn Error>> {
        let (flr, flrpara) = (Flr::default(), FlrPara::default());
        let nerva: Nerva<Flr, FlrPara> = Nerva::new(flr, flrpara);
        let resp = nerva.get()?;
        println!("{:?}", resp);
        return Ok(());
    }
}
