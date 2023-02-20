pub use crate::core::{Client, Params};

/// Util for handling API keys
pub mod keys
{
    /// Loads an API key from .env file with key NASA_API_KEY
    pub fn load() -> Result<String, Box<dyn std::error::Error>>
    {
        dotenv::dotenv()?;
        std::env::var("NASA_API_KEY").map_err(|_| "NASA_API_KEY not found in .env file".into())
    }
    /// Loads an API key from .env file with key NASA_API_KEY and appends it to the query
    pub fn include(query: &str) -> Result<String, Box<dyn std::error::Error>>
    {
        let key = load()?;
        return Ok(format!("{}&api_key={}", query, key));
    }
}

/// Re-exports client parameters
pub mod params
{
    pub use crate::clients::apod::ApodParams;
    pub use crate::clients::neo::{feed::NeoFeedParams, lookup::NeoLookupParams};
}


#[cfg(test)]
mod test
{
    use super::keys;

    #[test]
    fn load_key()
    {
        let key = keys::load();
        assert!(key.is_ok());
        println!("Key: {}", key.unwrap());
    }
}
