use crate::prelude::params::DefaultParams;
use crate::prelude::{Params, SubClient};

/// Params for the SEP API
pub type SEPParams<'p> = DefaultParams<'p>;

/// SEP API client
#[derive(Debug, Clone)]
pub struct SEP {}

impl Default for SEP {
    fn default() -> Self {
        SEP {}
    }
}

#[allow(missing_docs)]
impl SEP {
    pub fn new() -> Self {
        SEP::default()
    }
}

impl<'p, PARAMS> SubClient<PARAMS> for SEP
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/SEP";
}

#[cfg(test)]
mod tests {
    use super::{SEPParams as SepParams, SEP as Sep};
    use crate::prelude::__x::*;

    #[test]
    fn test_sep() {
        let (sep, seppara) = (Sep::default(), SepParams::default());
        let nerva: Nerva<Sep, SepParams> = Nerva::new(sep, seppara);
        let resp = nerva.get();
        println!("{:?}", resp);
    }
}
