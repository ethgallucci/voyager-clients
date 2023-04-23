use crate::prelude::params::DefaultParams;
use crate::prelude::{Params, SubClient};

/// Params for the RBE API
pub type RBEParams<'p> = DefaultParams<'p>;

/// RBE API client
#[derive(Debug, Clone)]
pub struct RBE {}

impl Default for RBE {
    fn default() -> Self {
        Self {}
    }
}

#[allow(missing_docs)]
impl RBE {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'p, DEP> SubClient<DEP> for RBE
where
    DEP: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/RBE";
}

#[cfg(test)]
mod rbe_tests {
    use super::{RBEParams as RbePara, RBE as Rbe};
    use crate::prelude::__x::*;

    #[test]
    fn test_rbe() {
        let (rbe, rbepara) = (Rbe::default(), RbePara::default());
        let nerva: Nerva<Rbe, RbePara> = Nerva::new(rbe, rbepara);
        let resp = nerva.get();
        println!("{:?}", resp);
    }
}
