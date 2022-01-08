use std::env;
mod lib;
use lib::*;

mod argparse;
use argparse::*;

fn main() {
    // Collect the arguments
    let args: Vec<String> = env::args().collect();
    let command = argparse().unwrap();
    // check if the command is a config command
    if command == Arg::SETKEY {
        // Set the API key
        keys::set_key(&args[3]).unwrap();
    } else if command == Arg::GETKEY {
        let key = keys::get_key();
        println!("key: {:?}", key.unwrap());
    }
    // Check if the command is a help command
    else if command == Arg::HELP {
        println!("\nconfig directory can be found at /Users/<Username>/voyager");
        println!("api key can be found at the config directory in .api_key.txt");
        println!("\nCommands:\nset key [key] -> stores API key in config directory");
        println!("get key -> retrieves API key from config directory");
    }
    // Command is not a config command
    else {
        println!("Command is not a recognized config command");
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
        use voyager_client::donki_client::*;

        // Setup timing
        let start = timing::one_week();
        let end = timing::today();
        // Instantiate base
        let base = Solar::new();
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
        use voyager_client::neo_client::*;

        let start = String::from("2022-01-01");
        let end = timing::today();
        // Instantiate base
        let base = Neo::new(start, end);
        // Try query
        base.query().unwrap();
    }

    #[test]
    fn try_insight() {
        use voyager_client::insight::*;

        let base = InsightWeather::new();
        base.query().unwrap();
    }

    #[test]
    fn try_cme() {
        use voyager_client::donki_client::*;

        let base = CoronalMassEjection::new();

        let start = String::from("2022-01-01");
        let end = timing::today();
        base.query(start, end).unwrap();
    }
}
