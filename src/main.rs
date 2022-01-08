mod lib;
use lib::*;


fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_apod() {
        // Instantiate base
        let mut base = Apod::new();
        // Try to set the date for query
        base.set_date(String::from("2015-06-07"));
        // Try query
        base.query().unwrap();
    }

    #[test]
    fn try_solar() {
        // Setup timing
        let start = timing::one_week();
        let end = timing::today();
        // Instantiate base
        let mut base = Solar::new(start, end);
        // Change start date by one month
        base.set_start(timing::one_month());
        // Try query
        base.query().unwrap();
    }

    #[test]
    fn try_magnetic() {
        // Setup timing
        let start = String::from("2019-01-01");
        let end = String::from("2022-01-01");
        // Instantiate base
        let base = Magnetic::new(start, end);
        // Try query
        base.query().unwrap();
    }
}
