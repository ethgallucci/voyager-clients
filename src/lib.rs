use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::error::Error;

pub mod client;

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

pub mod acl {
    use super::*;
    use client::{Base, BaseLayer};

    pub trait OpenApiClient {
        fn query(&self) -> Result<JsonValue, Box<dyn Error>>;
        fn into_interface<T: OpenApiClient>(acl: T) -> interface::Interface<T>;
    }

    pub fn fetch_defaults(url: &String) -> Result<JsonValue, ()> {
        let res: String = ureq::get(url).call().unwrap().into_string().unwrap();

        let try_json = serde_json::from_str(&res).unwrap();
        Ok(try_json) 
    }

    pub mod apod {
        use super::*;
        use serde_json::Value as JsonValue;

        #[derive(Debug, Clone, PartialEq)]
        pub struct ApodClient {
            pub bcl: Base,
            pub query: Option<ApodQuery>,
            pub date: Option<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum ApodQuery {
            Default,
            WithDate { date: String },
        }

        impl ApodClient {
            pub fn new() -> ApodClient {
                ApodClient {
                    bcl: Base {
                        burl: String::from("https://api.nasa.gov/planetary/apod?"),
                        lastr: None,
                    },
                    query: Some(ApodQuery::Default),
                    date: None,
                }
            }

            pub fn setparams(&mut self, apod_query: ApodQuery) -> () {
                self.query = Some(apod_query)
            }
        }

        impl OpenApiClient for ApodClient {
            fn query(&self) -> Result<JsonValue, Box<dyn Error>> {
                let key = keys::from_dotenv().unwrap();
                assert!(self.query.is_some());
                let url: String = match self.query.clone().unwrap() {
                    ApodQuery::Default => {
                        format!("{}api_key={}", self.bcl.burl, key)
                    }
                    ApodQuery::WithDate { date } => {
                        format!("{}date={}&api_key={}", self.bcl.burl, date, key)
                    }
                };

                let json: JsonValue = fetch_defaults(&url).unwrap();
                Ok(json)
            }
            fn into_interface<T: OpenApiClient>(acl: T) -> interface::Interface<T> {
                interface::Interface {
                    acl,
                    queries_tried: None,
                    responses_recieved: None,
                    resmap: None,
                }
            }
        }
    }

    pub mod neo {
        use super::*;

        #[derive(Debug, Clone, PartialEq)]
        pub struct NeoClient {
            bcl: Base,
            query: Option<NeoQuery>,
            range: Option<(String, String)>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum NeoQuery {
            Default,
            WithRange { range: (String, String) }
        }
    
        impl NeoClient {
            pub fn new() -> NeoClient { unimplemented!() }
            pub fn set_range(&mut self, range: (String, String)) -> () {
                self.range = Some(range)
            }
        }

    }
}

pub mod interface {
        use super::*;
        use std::fmt::Debug;
        use std::collections::HashMap;

        use acl::OpenApiClient;

        pub type ResMap = HashMap<String, String>;
        pub trait HasResMap<T> 
        where
            T: OpenApiClient {
            fn make_resmap(iface: Interface<T>) -> Result<ResMap, ()>;
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Interface<T: OpenApiClient> {
            pub acl: T,
            pub queries_tried: Option<u32>,
            pub responses_recieved: Option<u32>,
            pub resmap: Option<ResMap>,
        }

        impl<T> Interface<T>
        where
            T: OpenApiClient {
            pub fn new(acl: T) -> Self {
                Interface { 
                    acl,
                    queries_tried: None,
                    responses_recieved: None,
                    resmap: None,
                }
            } 

            pub fn swap_acl(&mut self, acl: T) -> () {
                self.acl = acl
            }
        }

    }
