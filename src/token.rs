use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{BufReader, Read, BufWriter, Write};

pub fn load_token() -> Option<String> {
    // println!("Loading token...");

    // exists path?
    if let Some(mut home) = env::home_dir() {
        home.push(".bitly-rs/");
        if home.exists() {
            home.push("token");
            if home.exists() {
                return read_token_from_file(&home);
            } else {
                return add_token(&home);
            }
        } else {
            // TODO is cloning here okay?
            fs::create_dir(home.clone()).unwrap();
            home.push("token");
            return add_token(&home);
        }
    } else {
        println!("Could not find your home directory!");
        return None;
    }
}


fn read_token_from_file(token_file: &Path) -> Option<String> {
    let mut text = String::new();
    let mut reader = match File::open(token_file) {
        Ok(handle) => BufReader::new(handle),
        Err(e)     => {
            println!("Could not read from the token file: {}", e);
            return None;
        }
    };

    match reader.read_to_string(&mut text) {
        Ok(_)  => Some(text.trim().to_string()),
        Err(_) => None
    }
}


fn add_token(token_file: &Path) -> Option<String> {
    let mut bitly_token = String::new();
    println!("Please enter your Bit.ly token:");
    match io::stdin().read_line(&mut bitly_token) {
        Ok(_)  => (),
        Err(_) => return None
    };

    let mut writer = match File::create(token_file) {
        Ok(handle) => BufWriter::new(handle),
        Err(_)     => {
            println!("Could not open the file!");
            return None;
        }
    };

    match writer.write(&bitly_token.trim().to_string().clone().into_bytes()) {
        Ok(_)  => (),
        Err(_) => println!("Unable to write to the file. Continuing without saving the token.")
    }

    Some(bitly_token)
}
