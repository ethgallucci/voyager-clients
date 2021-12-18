use std::error::Error;

use super::keys;
use super::to_pretty::to_string_pretty;

pub fn apod() -> Result<String, Box<dyn Error>> {
    let key: String = keys::get_key()?;
    let url: String = format!("https://api.nasa.gov/planetary/apod?api_key={}", key);

    let res: String = ureq::get(&url).call()?.into_string()?;
    let apod = to_string_pretty(res).unwrap();

    Ok(apod)
}