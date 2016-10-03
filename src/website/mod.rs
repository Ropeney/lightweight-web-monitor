extern crate hyper;
extern crate time;

use self::hyper::client::{Client};
use self::time::PreciseTime;

use notification_queue;

pub fn check_site(url: &str) -> Box<notification_queue::Response> {
    let client = Client::new();

    let start = PreciseTime::now();
    let res = client.get(url).send().unwrap();
    let end = PreciseTime::now();

    Box::new(notification_queue::Response { time: start.to(end).num_milliseconds(),
        state: res.status.to_string(), identifier: url.to_string() })
}

#[test]
pub fn check_site_returns_valid_response() {
    let response = check_site("http://google.com");
    assert_eq!(response.state, "200 OK");
    assert_eq!(response.identifier, "http://google.com");
    assert_eq!(response.time > 0, true);
}
