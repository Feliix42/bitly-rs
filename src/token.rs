use std::fs::File;
use std::io::{BufReader, Read};

pub fn load_token() -> Option<String> {
    // println!("Loading token...");

    let mut text = String::new();
    let mut reader = match File::open("access_token.txt") {
        Ok(handle) => BufReader::new(handle),
        Err(e)     => panic!("Error: {}", e)
    };

    match reader.read_to_string(&mut text) {
        Ok(_)  => Some(text.trim().to_string()),
        Err(_) => None
    }
}
