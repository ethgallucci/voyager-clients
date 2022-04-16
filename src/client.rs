use serde_json::Value as JsonValue;

/// A base API client. Contains no abstraction to the Open APIs, but contains the base url to
/// the Open API endpoint, and the last response recieved if there was any.
#[derive(Debug, Clone, PartialEq)]
pub struct Base {
    pub burl: String,
    pub lastr: Option<JsonValue>,
}

/// Defines behavior of the Base struct
pub trait BaseLayer {
    fn burl_update(&mut self, burl: String) -> ();
    fn lastr(&self) -> Option<JsonValue>;
}

impl BaseLayer for Base {
    fn burl_update(&mut self, burl: String) -> () {
        self.burl = burl
    }

    fn lastr(&self) -> Option<JsonValue> {
        self.lastr.clone()
    }
}
