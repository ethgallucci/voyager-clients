use std::error::Error;

use super::timing;
use super::keys;
use super::to_pretty::to_string_pretty;

pub fn neo() -> Result<String, Box<dyn Error>> {
    let start = timing::one_day();
    let now = timing::today();
    println!("Starting query from {} to {}", start, now);
    
    let key = keys::get_key()?;
    let url = format!("https://api.nasa.gov/neo/rest/v1/feed?start_date={}&end_date={}&api_key={}", start, now, key);
    
    let res: String = ureq::get(&url).call()?.into_string()?;
    let neo = to_string_pretty(res).unwrap();

    Ok(neo)
}
