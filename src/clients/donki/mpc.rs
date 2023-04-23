use crate::prelude::params::DefaultParams;
use crate::prelude::{Params, SubClient};

/// Params for the MPC API
pub type MPCParams<'p> = DefaultParams<'p>;

/// The `MPC` client
#[derive(Debug, Clone)]
pub struct MPC {}

impl Default for MPC {
    fn default() -> Self {
        Self {}
    }
}

impl MPC {
    /// Create a new `MPC` client
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'p, PARAMS> SubClient<PARAMS> for MPC
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/MPC";
}

#[cfg(test)]
mod mpc_tests {
    use super::{MPCParams, MPC};
    use crate::prelude::__x::*;

    #[test]
    fn test_mpc() {
        let (mpc, mpcpara) = (MPC::default(), MPCParams::default());
        let nerva: Nerva<MPC, MPCParams> = Nerva::new(mpc, mpcpara);
        let resp = nerva.get();
        println!("{:?}", resp);
    }
}
