use std::error::Error;

use super::keys;
use super::to_pretty::to_string_pretty;
use super::timing;

pub fn sflare() -> Result<String, Box<dyn Error>> {
    let now = timing::today();
    let start = timing::one_month();
    println!("Starting query from {} to {}", start, now);
    
    let key: String = keys::get_key().unwrap();
    let url: String = format!("https://api.nasa.gov/DONKI/FLR?startDate={}&endDate={}&api_key={}", start, now, key);
    
    let res: String = ureq::get(&url).call()?.into_string()?;
    let sflare = to_string_pretty(res).unwrap();
    
    Ok(sflare)
}

pub fn magnetic() -> Result<String, Box<dyn Error>> {
    let key: String = keys::get_key().unwrap();
    let url: String = format!("https://api.nasa.gov/DONKI/GST?startDate=2021-01-01&endDate=2021-12-10&api_key={}", key);
    
    let res: String = ureq::get(&url).call()?.into_string()?;
    let magnetic = to_string_pretty(res).unwrap();

    Ok(magnetic)
}