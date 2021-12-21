use std::env;

#[derive(Debug, PartialEq)]
pub enum Arg {
    APOD,
    NEO,
    MAG,
    SFLARE,
    BADCOMMAND,
    SETKEY,
    GETKEY,
}

pub fn argparse() -> Result<Arg, ()> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "Expected at least one argument");

    match &args[1] as &str {
        "apod" => Ok(Arg::APOD),
        "neo" => Ok(Arg::NEO),
        "magnetic" => Ok(Arg::MAG),
        "flare" => Ok(Arg::SFLARE),
        "set" => match &args[2] as &str {
            "key" => Ok(Arg::SETKEY),
            _ => Ok(Arg::BADCOMMAND)
        },
        "get" => match &args[2] as &str {
            "key" => Ok(Arg::GETKEY),
            _ => Ok(Arg::BADCOMMAND)
        },
        _ => Ok(Arg::BADCOMMAND)
    }
}
