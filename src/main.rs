mod lib;
use lib::*;

fn main() {
    
}

#[test]
fn try_apod() {
    let mut base = apod::new();
    base.set_date(String::from("2015-06-07"));
    let pic = base.query().unwrap();
    println!("{}", pic);
}
