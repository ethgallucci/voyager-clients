pub mod keys {
    use dotenv;
    use std::error::Error;

    pub fn from_dotenv() -> Result<String, Box<dyn Error>> {
        dotenv::dotenv().ok();

        let key = "API_KEY";
        let value = dotenv::var(key)?;
        Ok(value)
    }
}