extern crate hyper;
extern crate url;

use std::fs::File;
use std::io::{BufReader, Read};
use hyper::{Url, Client};
use url::percent_encoding;


fn load_token() -> Option<String> {
    println!("Loading token...");

    let mut text = String::new();
    let mut reader = match File::open("access_token.txt") {
        Ok(handle) => BufReader::new(handle),
        Err(e)     => panic!("Error: {}", e)
    };

    match reader.read_to_string(&mut text) {
        Ok(_)  => return Some(text.trim().to_string()),
        Err(_) => return None
    };
}


fn get_url() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not read input.");
    input.trim().to_string()
}


fn encode_url(long_url: String) -> String {
    percent_encoding::percent_encode(
        &long_url.into_bytes(),
        percent_encoding::PATH_SEGMENT_ENCODE_SET
    ).collect::<String>()
}


fn shorten(token: String, long_url: String) -> String {
    // TODO: Error Handling?!
    let url = format!(
        "https://api-ssl.bitly.com/v3/shorten?longUrl={url}&access_token={token}&format=txt",
        url = long_url,
        token = token
    );

    println!("URL: {}", url);

    let client = Client::new();
    let mut response = client.get(&url).send().unwrap();

    let mut short_url = String::new();
    response.read_to_string(&mut short_url);
    short_url
}


fn main() {
    let token = load_token().unwrap();
    println!("Using token '{}'.", token);

    println!("{}", shorten(token, encode_url(get_url())));

    // take URL as input

    // Remove forbidden chars

    // make API call

    // return :)
}
