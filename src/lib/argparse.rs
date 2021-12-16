use std::env;

#[derive(Debug, PartialEq)]
pub enum Arg {
    APOD,
    NEO,
    MAG,
    SFLARE,
    EXO,
    BADCOMMAND
}

pub fn argparse() -> Result<Arg, ()> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Expected at least one argument");

    match &args[1] as &str {
        "apod" => Ok(Arg::APOD),
        "neo" => Ok(Arg::NEO),
        "magnetic" => Ok(Arg::MAG),
        "flare" => Ok(Arg::SFLARE),
        "exo" => Ok(Arg::EXO),
        _ => Ok(Arg::BADCOMMAND)
    }
}
