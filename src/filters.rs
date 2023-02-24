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
        // COLLECT ALL VALUES THAT MATCH THE KEY
        let key = self.key.clone();
        let mut result = Vec::new();
        let mut stack = Vec::new();
        // iterate over the json and collect all values that match the key
        stack.push(json);
        while let Some(value) = stack.pop()
        {
            match value
            {
                serde_json::Value::Object(map) =>
                {
                    for (k, v) in map
                    {
                        if k == key
                        {
                            result.push(v);
                        }
                        else
                        {
                            stack.push(v);
                        }
                    }
                }
                serde_json::Value::Array(array) =>
                {
                    for v in array
                    {
                        stack.push(v);
                    }
                }
                _ =>
                {}
            }
        }

        Ok(result)
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

    #[test]
    fn test_filters()
    {
        /*
        let apod = Apod::default();
        let response = apod.get(ApodParams::Date("2023-02-21"));
        let values = filter::<Match<String>>(response.unwrap(), &Match::new("explanation"));
        assert!(values.is_ok());
        println!("{:#?}", values.unwrap());
        */

        use crate::clients::donki::flr::*;
        let flr = FLR::default();
        let params = FLRParams::StartDate("2023-01-01");
        let response = flr.get(params).unwrap();
        println!("{:#?}", response);
        let values = filter::<Match<u32>>(response, &Match::new("activeRegionNum"));
        println!("{:#?}", values.unwrap());
    }
}
