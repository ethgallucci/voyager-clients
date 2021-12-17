use std::error::Error;

use super::keys;
use super::to_pretty::to_string_pretty;

pub fn sflare() -> Result<String, Box<dyn Error>> {
    let key: String = keys::getKey();
    let url: String = format!("https://api.nasa.gov/DONKI/FLR?startDate=2021-01-01&endDate=2021-12-15&api_key={}", key);
    
    let res: String = ureq::get(&url).call()?.into_string()?;
    let sflare = to_string_pretty(res).unwrap();
    
    Ok(sflare)
}

pub fn magnetic() -> Result<String, Box<dyn Error>> {
    let key: String = keys::getKey();
    let url: String = format!("https://api.nasa.gov/DONKI/GST?startDate=2021-01-01&endDate=2021-12-10&api_key={}", key);
    
    let res: String = ureq::get(&url).call()?.into_string()?;
    let magnetic = to_string_pretty(res).unwrap();

    Ok(magnetic)
}