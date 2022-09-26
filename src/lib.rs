pub mod client;
pub mod custom;

#[allow(dead_code)]
pub(crate) mod key {
    pub fn load(k: &str) -> String {
        dotenv::dotenv().ok();
        std::env::var(k).expect(&format!("{} is not set", k))
    }
}
