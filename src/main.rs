use std::error::Error;
use std::env;

mod argparse;
use argparse::*;

mod lib;
use lib::*;

fn main() {
    let mut base = apod::new();
    base.set_date(String::from("2015-06-06"));
    let pic = base.query().unwrap();

    println!("{}", pic);
}
