#[cfg(test)]
mod test {
    #[test]
    fn doc_test() {
        use voyager_client::{donki, time};

        // Instantiate a base client
        let base = donki::GeoMagnetic::new();
        
        // Setup time
        let start = String::from("2015-01-01");
        let end = time::today();
        
        // Query the endpoint
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_apod() {
        use voyager_client::apod;
        use voyager_client::response::*;

        // Instantiate base
        let mut base = apod::ApodClient::new();
        // Try to set the date for query
        base.set_date(String::from("2021-06-07"));
        // Try query
        let res: Response = base.query().unwrap();

        let json = res.json();
        println!("{}", json.unwrap());
    }

    #[test]
    fn try_solar() {
        use voyager_client::donki;
        use voyager_client::time;

        // Setup time
        let start = time::one_month();
        let end = time::today();
        // Instantiate base
        let base = donki::SolarFlare::new();
        // Try query
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_magnetic() {
        use voyager_client::donki::*;

        // Setup time
        let start = String::from("2019-01-01");
        let end = String::from("2022-01-01");
        // Instantiate base
        let base = GeoMagnetic::new();
        // Try query
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_neo() {
        use voyager_client::neo;
        use voyager_client::time;

        let start = String::from("2022-01-01");
        let end = time::today();
        // Instantiate base
        let base = neo::Neo::new();
        // Try query
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_insight() {
        use voyager_client::insight;

        let base = insight::InsightWeather::new();
        base.query().unwrap();
    }

    #[test]
    fn try_cme() {
        use voyager_client::time;
        use voyager_client::donki::*;

        let base = CoronalMassEjection::new();

        let start = String::from("2022-01-01");
        let end = time::today();
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_env_keys() {
        use voyager_client::key;

        let key = key::from_dotenv().unwrap();
        println!("{}", key);
    }

    #[test]
    fn try_tech_transfer_patent() {
        use voyager_client::tech;

        let base = tech::TechTransferClient::new();

        let query = String::from("engine");
        base.query(query).unwrap();
    }

    #[test]
    fn try_tech_transfer_software() {
        use voyager_client::tech;
        use tech::Collections;

        let mut base = tech::TechTransferClient::new();
        base.switch(Collections::Software).unwrap();

        let query = String::from("engine");
        base.query(query).unwrap();
    }

    #[test]
    fn try_fireball() {
        use voyager_client::jpl::*;

        let mut base = FireballClient::new();
        base.limit(1);

        base.query().unwrap();
    }

    #[test]
    fn try_default_mission_design() {
        use voyager_client::jpl::*;

        let base = MissionDesign::new();
        base.query(QueryType::DES, "2012%20TC4").unwrap();
    }

    #[test]
    fn try_mission_design_accessible_lim_crit_year() {
        use voyager_client::jpl::*;

        let mut base = MissionDesignAccessible::new();
        base.limit(10);
        base.crit(1);
        base.year(String::from("2025,2026,2027,2028,2029"));

        base.lim_crit_year().unwrap();
    }

    #[test]
    fn try_mission_design_map_mode() {
        use voyager_client::jpl::*;

        let mut base = MissionDesignMap::new();
        base.designation("2012%20TC4");
        base.mjd(58490);
        base.span(3652);
        base.tof(10, 36);
        base.step(2);

        base.query().unwrap();
    }

    #[test]
    fn try_solar_energetic_particle() {
        use voyager_client::donki::*;

        let base = SolarEnergeticParticle::new();
        let start = "2021-09-12".to_string();
        let end = "2022-01-11".to_string();

        base.query(start, end).unwrap();
    }
}