use std::error::Error;

pub fn apod() -> Result<String, Box<dyn Error>> {
    let apod: String = ureq::get("https://api.nasa.gov/planetary/apod?api_key=qxUB72hYa6OfctXigKLdRhATtGM9iCOKCKuBhQ19").call()?.into_string()?;

    Ok(apod)
}