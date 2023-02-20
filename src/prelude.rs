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

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum DefaultParams<'p>
    {
        StartDate(&'p str),
        EndDate(&'p str),
    }

    impl<'p> Default for DefaultParams<'p>
    {
        fn default() -> Self
        {
            Self::StartDate("2020-01-01")
        }
    }

    impl<'p> Into<String> for DefaultParams<'p>
    {
        fn into(self) -> String
        {
            match self
            {
                Self::StartDate(date) => format!("startDate={}", date),
                Self::EndDate(date) => format!("endDate={}", date),
            }
        }
    }

    impl<'p> crate::core::Params for DefaultParams<'p> {}
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
