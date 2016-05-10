extern crate hyper;
extern crate url;

use std::io::Read;
use hyper::Client;
use url::percent_encoding;


mod token;
use token::load_token;


fn get_url() -> String { //optional/result
    // wrap mit try!
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read input.");
    input.trim().to_string()
}


fn encode_url(long_url: &str) -> String {
    percent_encoding::percent_encode(
        long_url.as_bytes(),
        percent_encoding::PATH_SEGMENT_ENCODE_SET
    ).collect::<String>()
}


fn shorten(token: &str, long_url: &str) -> String {
    // TODO: Error Handling?!
    let url = format!(
        "https://api-ssl.bitly.com/v3/shorten?longUrl={url}&access_token={token}&format=txt",
        url = long_url,
        token = token
    );

    // println!("URL: {}", url);

    let client = Client::new();
    let mut response = client.get(&url).send().unwrap();

    let mut short_url = String::new();
    // TODO: Check result of the following operation
    response.read_to_string(&mut short_url).unwrap();
    short_url
}


fn main() {
    let token = load_token().unwrap();
    // println!("Using token '{}'.", token);

    println!("{}", shorten(&token, &encode_url(&get_url())).trim());
}
