use std::error::Error;

pub fn exoplanet() -> Result<String, Box<dyn Error>> {
    let exo: String = ureq::get("https://exoplanetarchive.ipac.caltech.edu/cgi-bin/nstedAPI/nph-nstedAPI?table=cumulative&where=koi_disposition%20like%20%27CANDIDATE%27&where=koi_period%3E300,koi_prad%3C2&order=koi_period&format=json").call()?.into_string()?;

    Ok(exo)
}
