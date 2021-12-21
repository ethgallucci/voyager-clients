use std::fs;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use users::{ get_user_by_uid, get_current_uid };

pub fn set_key(key: &str) -> std::io::Result<()> {
    let username = get_user();

    let path_to_voyager = format!("/Users/{}/voyager", username);
    std::fs::create_dir(&path_to_voyager).unwrap();

    let path_to_key = format!("{}/.api_key.txt", &path_to_voyager);
    let mut file = File::create(path_to_key)?;
    file.write_all(&key.as_bytes())?;
    println!("Set key for {}", username);
    Ok(())
}


pub fn get_key() -> Result<String, Box<dyn Error>> {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let path_to_key = format!("/Users/{}/voyager/.api_key.txt", user.name().to_string_lossy());
    let key = fs::read_to_string(&path_to_key)
        .expect("Couldn't read api key");

    Ok(key)
}

pub fn get_user() -> String {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let username = user.name().to_string_lossy();
    username.to_string()
}