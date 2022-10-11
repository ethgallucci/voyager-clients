#![cfg(test)]

#[test]
fn readme() {
    use nerva::apis::apod::*;
    use nerva::prelude::*;

    fn main() -> Result<(), anyhow::Error> {
        let apod = ApodClient {};
        // get today's entry in the Picture of the Day API
        match apod.get(ApodParams::default()) {
            Ok(response) => {
                println!("got response!: {}", response);
            }
            Err(e) => return Err(e),
        }

        // fetch a specific entry
        let date = "2022-09-01".to_string();
        match apod.get(ApodParams::Date(date.clone())) {
            Ok(response) => {
                println!("entry for {}:\n{}", date, response);
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

    assert!(matches!(main(), Ok(())))
}
