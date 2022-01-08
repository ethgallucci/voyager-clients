use std::env;

#[derive(Debug, PartialEq)]
pub enum Arg {
    BADCOMMAND,
    SETKEY,
    GETKEY,
    HELP
}

pub fn argparse() -> Result<Arg, ()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        return Ok(Arg::HELP)
    }

    match &args[1] as &str {
        "set" => match &args[2] as &str {
            "key" => Ok(Arg::SETKEY),
            _ => Ok(Arg::BADCOMMAND)
        },
        "get" => match &args[2] as &str {
            "key" => Ok(Arg::GETKEY),
            _ => Ok(Arg::BADCOMMAND)
        },
        "help" => Ok(Arg::HELP),
        _ => Ok(Arg::BADCOMMAND)
    }
}
