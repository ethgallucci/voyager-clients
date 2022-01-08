mod lib;
use lib::*;

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_apod() {
        // Instantiate base
        let mut base = apod::new();
        // Try to set the date for query
        base.set_date(String::from("2015-06-07"));
        // Try query
        let pic = base.query().unwrap();
        println!("{}", pic);
    }

    #[test]
    fn try_flare() {
        // Setup timing
        let start = timing::one_week();
        let end = timing::today();
        // Instantiate base
        let mut base = solar::new(start, end);
        // Change start date by one month
        base.start(timing::one_month());
        // Try query
        let solar = base.query().unwrap();
        println!("{}", solar);
    }
}
