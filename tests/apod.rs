#![cfg(test)]
#![feature(stmt_expr_attributes)]
mod apod_test {
    use nerva::apis::apod::*;
    use nerva::prelude::*;
    #[test]
    fn test_default_ok() {
        pretty_env_logger::try_init().ok();
        let today = chrono::Utc::today().format("%Y-%m-%d").to_string();
        assert_eq!(
            ApodParams::default(),
            ApodParams::Date(today),
            "Default params should be today"
        );
    }
    
    fn test_get() {
        pretty_env_logger::try_init().ok();

        let apod = ApodClient {};
        #[rustfmt::skip]
        match apod.get(ApodParams::default()) {
            Ok(s) => { log::info!("Apod test passed: dumping response"); log::debug!("{}", s); },
            Err(e) => { log::error!("get test failed! {:?}", e); panic!() },
        }
    }
}
