use std::error::Error;

mod lib;

use lib::argparse::*;
use lib::apod::*;
use lib::neo::*;
use lib::exoplanet::*;
use lib::weather::*;

fn main() -> Result<(), Box<dyn Error>> {

    let command = argparse().unwrap();

    let res: String;
    match command {
        Arg::APOD => res = apod().unwrap(),
        Arg::NEO => res = neo().unwrap(),
        Arg::MAG => res = magnetic().unwrap(),
        Arg::SFLARE => res = sflare().unwrap(),
        Arg::EXO => res = exoplanet().unwrap(),
        // Default to apod if command can't be parsed
        Arg::BADCOMMAND => res = apod().unwrap(),
    }

    if command == Arg::BADCOMMAND {
        println!("Defaulting to APOD upon Bad Command\n\n")
    }
    println!("response: {}", res);

    Ok(())
}
