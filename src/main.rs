extern crate hyper;
extern crate url;


use std::env;
use std::io::Read;
use hyper::Client;
use hyper::error::Error;
use hyper::status::StatusCode;
use url::percent_encoding;

mod token;

use token::load_token;


fn encode_url(long_url: &str) -> String {
    percent_encoding::percent_encode(
        long_url.as_bytes(),
        percent_encoding::PATH_SEGMENT_ENCODE_SET
    ).collect::<String>()
}


fn shorten(token: &str, long_url: &str) -> String {
    let url = format!(
        "https://api-ssl.bitly.com/v3/shorten?longUrl={url}&access_token={token}&format=txt",
        url = long_url,
        token = token
    );

    let client = Client::new();
    let mut response = match client.get(&url).send() {
        Ok(resp)          => resp,
        Err(Error::Io(_)) => return "Could not access network. Make sure you have an active network connection.".to_string(),
        Err(e)            => return format!("An error occured: {}", e),
    };

    let mut short_url = String::new();
    response.read_to_string(&mut short_url).unwrap();

    match response.status {
        StatusCode::Ok                  => short_url,
        StatusCode::InternalServerError => match short_url.trim() {
            "INVALID_URI" => "The URL you entered is invalid.".to_string(),
            "INVALID_ARG_ACCESS_TOKEN" => "Your access token is invalid or expired.".to_string(),
            _                          => format!("An error occured: {}", short_url.trim()),
        },
        _                               => format!("A fatal error occured: {}", response.status),
    }
}


fn main() {
    let token = match load_token() {
        Some(t) => t,
        None    => return
    };

    if let Some(url) = env::args().nth(1) {
        println!("{short}", short = shorten(&token, &encode_url(&url)).trim());
    } else {
        println!("Usage: bitly [URL]");
    }

}
