mod lib;

fn main() {
    println!("Voyager v0.2.5")
}

#[cfg(test)]
mod test {

    #[test]
    fn doc_test() {
        use voyager_client::{donki_client, timing};

        // Instantiate a Base Client
        let base_donki_client = donki_client::Solar::new();

        // Setup timing parameters
        let start = String::from("2018-01-01");
        let end = timing::today();

        // Query the API
        base_donki_client.query(start, end).unwrap();
    }

    #[test]
    fn try_apod() {
        use voyager_client::apod_client::*;

        // Instantiate base
        let mut base = Apod::new();
        // Try to set the date for query
        base.set_date(String::from("2015-06-07"));
        // Try query
        base.query().unwrap();
    }

    #[test]
    fn try_solar() {
        use voyager_client::{donki_client, timing};

        // Setup timing
        let start = timing::one_month();
        let end = timing::today();
        // Instantiate base
        let base = donki_client::Solar::new();
        // Try query
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_magnetic() {
        use voyager_client::donki_client::*;

        // Setup timing
        let start = String::from("2019-01-01");
        let end = String::from("2022-01-01");
        // Instantiate base
        let base = Magnetic::new();
        // Try query
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_neo() {
        use voyager_client::{neo_client, timing};

        let start = String::from("2022-01-01");
        let end = timing::today();
        // Instantiate base
        let base = neo_client::Neo::new();
        // Try query
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_insight() {
        use voyager_client::insight_client::*;

        let base = InsightWeather::new();
        base.query().unwrap();
    }

    #[test]
    fn try_cme() {
        use voyager_client::timing;
        use voyager_client::donki_client::*;

        let base = CoronalMassEjection::new();

        let start = String::from("2022-01-01");
        let end = timing::today();
        base.query(start, end).unwrap();
    }

    #[test]
    fn try_env_keys() {
        use voyager_client::keys::from_dotenv;

        let key = from_dotenv().unwrap();
        println!("{}", key);
    }

    #[test]
    fn try_tech_transfer_patent() {
        use voyager_client::tech_transfer::*;

        let base = TechTransferClient::new();

        let query = String::from("engine");
        let res = base.query(query).unwrap();
        println!("{}", res);
    }

    #[test]
    fn try_tech_transfer_software() {
        use voyager_client::tech_transfer::*;

        let mut base = TechTransferClient::new();
        base.software();

        let query = String::from("engine");
        let res = base.query(query).unwrap();
        println!("{}", res);
    }
}
