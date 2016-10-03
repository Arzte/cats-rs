#![feature(rustc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate url;

use std::io::Read;
use url::Url;

pub fn fact(number: &str) -> String {
    // Construct the URL you want to access
    let url = format!("http://catfacts-api.appspot.com/api/facts?number={}", number)
        .parse::<Url>()
        .expect("Unable to parse URL");

    // Initialize the Hyper client and make the request.
    let client = hyper::Client::new();
    let mut response = client.get(url).send().unwrap();

    // Initialize a string buffer, and read the response into it.
    let mut result = String::new();
    response.read_to_string(&mut result).unwrap();

    // Deserialize the result.
    #[derive(Deserialize)]
    pub struct CatFacts {
        pub facts: Vec<String>,
        pub success: bool,
    }
    serde_json::from_str::<CatFacts>(&result).unwrap().facts.pop().unwrap()
}
