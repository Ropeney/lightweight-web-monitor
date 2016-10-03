use std::thread;
use std::sync::{Arc, Mutex};

mod website;

fn main() {
    let sites = Arc::new(Mutex::new(vec![
        "http://eevblog.com",
        "http://ropeney.com",
        "http://yahoo.com",
        "http://gumtree.com.au",
        "http://ebay.com.au",
        "http://facebook.com",
        "http://google.com",
        "http://barrista.com"]));

    let mut children = vec![];

    for _ in 0..8 {
        let data = sites.clone();

        children.push(thread::spawn(move || {

        // This is so that the mutex unlocks quicker
        let url = {
            let mut data = data.lock().unwrap();
            data.pop().unwrap()
        };

        let response = website::check_site(url);

        println!("{} : {} : {}ms", response.url, response.code,
            response.time);
      }));
    }

    for child in children {
        let _ = child.join();
    }
}

