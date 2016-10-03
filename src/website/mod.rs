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

#[test]
pub fn check_site_returns_valid_response() {
    let response = check_site("http://google.com");
    assert_eq!(response.code, "200 OK");
    assert_eq!(response.url, "http://google.com");
    assert_eq!(response.time > 0, true);
}
