use std::error::Error;

use super::to_pretty::to_string_pretty;

pub fn exoplanet() -> Result<String, Box<dyn Error>> {
    let res: String = ureq::get("https://exoplanetarchive.ipac.caltech.edu/cgi-bin/nstedAPI/nph-nstedAPI?table=cumulative&where=koi_disposition%20like%20%27CANDIDATE%27&where=koi_period%3E300,koi_prad%3C2&order=koi_period&format=json").call()?.into_string()?;

    let exo: String = to_string_pretty(res).unwrap();
    
    Ok(exo)
}
