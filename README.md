# nerva [![CodeFactor](https://www.codefactor.io/repository/github/phasewalk1/nerva/badge)](https://www.codefactor.io/repository/github/phasewalk1/nerva) ![CI Status](https://img.shields.io/github/workflow/status/phasewalk1/nerva/Rust)

## Forward-facing API

```rust
extern crate nerva;
use nerva::prelude::*;
use nerva::apis::apod::*;

fn main() -> Result<(), anyhow::Error> {
    let apod = ApodClient { };
    // get today's entry in the Picture of the Day API
    match apod.get(ApodParams::default()) {
        Ok(response) => { println!("got response!: {}", response); return Ok(()) },
        Err(e) => { return Err(e) },
    }

    // fetch a specific entry
    let date = "2022-09-01".to_string();
    match apod.get(ApodParams::Date(date)) {
        Ok(response) => { 
            println!("entry for {}:\n{}", date, response); return Ok(())
        },
        Err(e) => { return Err(e) },
    }
}
```

## API Under the Hood: Library Design
### The OpenApiClient Interface
```rust
pub trait OpenApiClient {
    #[ doc = "accepted parameters" ]
    type Params;
    #[ doc = "base url" ]
    const CONNECTION: &'static str;
    
    #[ doc = "query method" ]
    fn get(&self, params: Self::Params) -> Result<String, anyhow::Error>;

```
This defines the interface for by which all OpenApi clients are implemented within _nerva_. `anyhow::Error` is just a placeholder here for now in the return type of `get`. It will likely be replaced by a trait-instance specific error type for bettor error handling in the future.

### Implementing the OpenApiClient Trait
First, to implement a client wrapper for an OpenApi, a base struct is made:
```rust
pub struct ApodClient { }
```
Then, we define the parameters we expect the Api to handle:
```rust
pub struct ApodParams {
    Date(String),
    StartDate(String),
    EndDate(String),
}
```
We can even define a __default__ value for our parameters:
```rust
use chrono::Utc;
impl Default for ApodParams {
    fn default() -> Self {
        let today = Utc::today().format("%Y-%m-%d").to_string();
        return ApodParams::Date(today)
    }
}
```
It helps to have a custom conversion function implemented for our parameters (into formatted query strings). By keeping this conversion implementation separate from our `get` implementation, we can use it elsewhere in our code (i.e. maybe if we want to write a CLI):
```rust
impl From<ApodParams> for String {
    fn from(params: ApodParams) -> String {
        match params {
            ApodParams::Date(d) => { return format!("date={}", d) },
            ...
        }
    }
}

```
Our ApodClient struct seems pretty well oriented now. Let's implement our OpenApiClient trait for it:
```rust
impl OpenApiClient for ApodClient {
    // API params
    type Params = ApodParams;
    // Endpoint
    const CONNECTION: &'static str = "https://api.nasa.gov/planetary/apod?";

    // Query w/ params
    fn get(&self, params: Self::Params) -> Result<String, anyhow::Error> {
        // Push our params and api key to the endpoint string
        let mut base_url = String::from(Self::CONNECTION);
        base_url.push_str(&format!("{}", String::from(params)));
        base_url.push_str(&format!("&api_key={}", crate::key::load("API_KEY")));

        // Send the request and return the response
        let response = ureq::get(&base_url).call();
        #[rustfmt::skip]
        match response {
            Ok(r) => { return Ok(r.into_string()?) },
            Err(e) => { return Err(anyhow::anyhow!(e)) },
        }
    }
}
```
