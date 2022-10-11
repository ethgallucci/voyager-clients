[warning] This crate is currently _unstable_ due to its use of `stmt_expr_attributes`


### Forward-facing API

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

### API Under the Hood: Library Design
```rust
pub trait OpenApiClient {
    type Params;
    const CONNECTION: &'static str;
    fn get(&self, params: Self::Params) -> Result<String, anyhow::Error>;

```
This defines the interface for by which all OpenApi clients are implemented within _nerva_. First, to implement a client wrapper for an OpenAApi, a base struct is made:
```rust
pub struct Apod { }
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
impl Default for ApodParams {
    use chrono::Utc;
    fn default() -> Self {
        return Ok(
    }
}
```

`anyhow::Error` is just a placeholder here for now in the return type of `get`. It will likely be replaced by a trait-instance specific error type for bettor error handling in the future.
