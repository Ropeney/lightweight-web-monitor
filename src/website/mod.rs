extern crate hyper;
extern crate time;

use self::hyper::client::{Client};
use self::time::PreciseTime;

pub struct Response {
    pub time: i64,
    pub url: String,
    pub code: String
}

pub fn check_site(url: &str) -> Box<Response> {
    let client = Client::new();

    let start = PreciseTime::now();
    let res = client.get(url).send().unwrap();
    let end = PreciseTime::now();

    Box::new(Response { time: start.to(end).num_milliseconds(),
        code: res.status.to_string(), url: url.to_string() })
}

