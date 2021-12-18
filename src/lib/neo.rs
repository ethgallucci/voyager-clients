use std::error::Error;
use super::keys;

use super::to_pretty::to_string_pretty;

pub fn neo() -> Result<String, Box<dyn Error>> {
    let key = keys::get_key()?;
    let url = format!("https://api.nasa.gov/neo/rest/v1/feed?start_date=2021-12-11&end_date=2021-12-15&api_key={}", key);
    
    let res: String = ureq::get(&url).call()?.into_string()?;
    let neo = to_string_pretty(res).unwrap();

    Ok(neo)
}
