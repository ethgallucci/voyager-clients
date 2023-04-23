use crate::prelude::{Params, SubClient};

/// Params for the GST API
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub enum GSTParams<'p> {
    StartDate(&'p str),
    EndDate(&'p str),
    Empty,
}

impl Default for GSTParams<'_> {
    fn default() -> Self {
        GSTParams::Empty
    }
}

impl<'p> Into<String> for GSTParams<'p> {
    fn into(self) -> String {
        match self {
            GSTParams::StartDate(date) => format!("startDate={}", date),
            GSTParams::EndDate(date) => format!("endDate={}", date),
            GSTParams::Empty => String::new(),
        }
    }
}

impl<'p> Params for GSTParams<'p> {}

/// GST API client
#[derive(Debug, Clone)]
pub struct GST {}

impl Default for GST {
    fn default() -> Self {
        GST {}
    }
}

#[allow(missing_docs)]
impl GST {
    pub fn new() -> GST {
        GST::default()
    }
}

impl<'p, PARAMS> SubClient<PARAMS> for GST
where
    PARAMS: Params,
{
    const BASE_URL: &'static str = "https://api.nasa.gov/DONKI/GST";
}

#[cfg(test)]
mod gst_test {
    use super::{GSTParams as GstPara, GST as Gst};
    use crate::prelude::__x::*;

    #[ignore]
    fn test_gst() {
        let (gst, gstpara) = (Gst::default(), GstPara::default());
        let nerva: Nerva<Gst, GstPara> = Nerva::new(gst, gstpara);
    }
}
