use crate::prelude::params::DefaultParams;
use crate::prelude::*;

/// Params for the IPS API
pub type IPSParams<'p> = DefaultParams<'p>;

/// IPS API client
#[derive(Debug, Clone)]
pub struct IPS {}

impl Default for IPS {
    fn default() -> Self {
        Self {}
    }
}

#[allow(missing_docs)]
impl IPS {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'p, PARAMS> SubClient<PARAMS> for IPS
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/IPS";
}

#[cfg(test)]
mod ips_test {
    use super::{IPSParams as IpsPara, IPS as Ips};
    use crate::prelude::__x::*;

    #[test]
    fn ips_test() {
        let (ips, ipspara) = (Ips::default(), IpsPara::default());
        let nerva: Nerva<Ips, IpsPara> = Nerva::new(ips, ipspara);
        let resp = nerva.get();
        println!("{:?}", resp);
    }
}
