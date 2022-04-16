#[cfg(test)]
mod test {
    use serde_json::Value as JsonValue;

    use voyager_client::*;
    use client::BaseLayer;
    use acl::OpenApiClient;

    #[test]
    fn can_use_bcl_trait_methods_on_acl() {
        let mut apod = acl::apod::ApodClient::new();
        let lastr = apod.bcl.lastr();
        // Break the base url
        apod.bcl.burl_update("lolwut.com".to_string())
    }

    #[test]
    fn can_build_apod_client() {
        let apod = acl::apod::ApodClient::new();
        let res: JsonValue = apod.query().unwrap();
        let pretty = serde_json::to_string_pretty(&res).unwrap();
        println!("{}", pretty)
    }

    #[test]
    fn can_build_interface() {
        use interface::*;
        use acl::{apod::ApodClient, OpenApiClient};

        let apod = ApodClient::new();

        #[allow(unused)]
        let apod_interface_1 = Interface::new(apod.clone());
        let apod_interface_2: Interface<ApodClient> = ApodClient::into_interface(apod);

        println!("{:?}", apod_interface_2)
    }
}
