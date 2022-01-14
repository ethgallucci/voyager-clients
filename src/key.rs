use dotenv;
use std::error::Error;

/// Method for retrieving keys from .env files.
/// * Takes no arguments
/// * Returns a Result<String, Box<dyn Error>>

pub fn from_dotenv() -> Result<String, Box<dyn Error>> {
    dotenv::dotenv().ok();

    let key = "API_KEY";
    let value = dotenv::var(key)?;
    Ok(value)
}
