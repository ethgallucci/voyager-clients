use std::fs;
use std::fs::File;
use std::io::prelude::*;
use users::{ get_user_by_uid, get_current_uid };

pub fn setKey(key: &str) -> std::io::Result<()> {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = user.name().to_string_lossy();
    println!("Setting key for {}", username);

    let path_to_voyager = format!("/Users/{}/voyager", username);
    std::fs::create_dir(&path_to_voyager).unwrap();

    let path_to_key = format!("{}/.api_key.txt", &path_to_voyager);
    let mut file = File::create(path_to_key)?;
    file.write_all(&key.as_bytes())?;
    Ok(())
}


pub fn getKey() -> String {
    let key = fs::read_to_string("/Users/ethangallucci/voyager/.api_key.txt")
        .expect("Couldn't read api key");

    key
}