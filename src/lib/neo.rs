use std::error::Error;

pub fn neo() -> Result<String, Box<dyn Error>> {
    let neo: String = ureq::get("https://api.nasa.gov/neo/rest/v1/feed?start_date=2021-12-11&end_date=2021-12-15&api_key=qxUB72hYa6OfctXigKLdRhATtGM9iCOKCKuBhQ19").call()?.into_string()?;

    Ok(neo)
}