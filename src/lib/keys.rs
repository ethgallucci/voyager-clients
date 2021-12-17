use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn setKey(key: &str) -> std::io::Result<()> {
    let mut file = File::create(".api_key.txt")?;
    file.write_all(&key.as_bytes())?;
    Ok(())
}


pub fn getKey() -> String {
    let key = fs::read_to_string(".api_key.txt")
        .expect("Couldn't read api key");

    key
}