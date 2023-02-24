#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, PartialEq)]
pub struct Match<T>
{
    key: String,
    values: Option<Vec<T>>,
}

impl<T> Match<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone + std::fmt::Debug + PartialEq,
{
    pub fn new(key: &str) -> Self
    {
        return Self {
            key: key.to_owned(),
            values: None,
        };
    }
}

impl<T> super::core::Filter for Match<T>
where
    T: serde::Serialize + serde::de::DeserializeOwned + Clone + std::fmt::Debug + PartialEq,
{
    fn filter(&self, json: serde_json::Value) -> Result<Vec<serde_json::Value>, anyhow::Error>
    {
        let mut filtered = Vec::new();
        let key = &self.key;
        let values = &self.values;
        match json
        {
            serde_json::Value::Array(arr) =>
            {
                for item in arr
                {
                    match item
                    {
                        _ =>
                        {}
                    }
                }
            }
            _ =>
            {
                let v = json.get(key).unwrap_or(&serde_json::Value::Null);
                if v.is_null()
                {
                    return Err(anyhow::anyhow!("Key not found"));
                }
                filtered.push(v.to_owned());
                return Ok(filtered);
            }
        }
        return Ok(filtered);
    }
}

pub fn filter<F>(
    res: serde_json::Value,
    filter: &F,
) -> Result<Vec<serde_json::Value>, anyhow::Error>
where
    F: super::core::Filter,
{
    return filter.filter(res);
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::clients::apod::{Apod, ApodParams};
    use crate::core::Client;
    use crate::core::Filter;

    #[test]
    fn test_apod()
    {
        let apod = Apod::default();
        let response = apod.get(ApodParams::default());
        let values = filter::<Match<String>>(response.unwrap(), &Match::new("explanation"));
        assert!(values.is_ok());
        println!("{:#?}", values.unwrap());
    }
}
