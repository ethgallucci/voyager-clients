use serde::{Deserialize, Serialize};

pub trait Client {
    fn base(&self) -> &BaseBind;
    fn make_query_str(&self, query: &str) -> String;
}

pub trait Interface {
    type Params;

    fn start(base: BaseBind) -> ClientBind;
    fn query(&self, params: Self::Params) -> String;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BaseBind(String);

impl BaseBind {
    pub fn url(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct ClientBind {
    base: BaseBind,
    target: Option<String>,
    /// Last (Query, Response)
    #[allow(dead_code)]
    lqr: Option<(String, String)>,
}

impl ClientBind {
    pub fn new(
        base: BaseBind,
        target: Option<String>,
        lqr: Option<(String, String)>,
    ) -> ClientBind {
        ClientBind { base, target, lqr }
    }

    pub fn new_base(&mut self, base: BaseBind) {
        self.base = base;
    }

    pub fn set_target(&mut self, target: Option<String>) {
        self.target = target;
    }
}

impl Client for ClientBind {
    fn base(&self) -> &BaseBind {
        &self.base
    }

    fn make_query_str(&self, query: &str) -> String {
        let mut url = self.base.url().to_string();
        url.push_str(&query);
        url
    }
}
