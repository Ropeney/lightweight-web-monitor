use std::thread;
use std::sync::{Arc, Mutex};

mod website;

fn main() {
    let sites: Arc<Mutex<Vec<(fn(url: &str) -> Box<website::Response>, &str)>>> = Arc::new(Mutex::new(vec![
        (website::check_site,"http://eevblog.com"),
        (website::check_site, "http://google.com"),
        ]));

    let mut children = vec![];

    for _ in 0..2 {
        let data = sites.clone();

        children.push(thread::spawn(move || {

        // This is so that the mutex unlocks quicker
        let (method, paramter) = {
            let mut data = data.lock().unwrap();
            data.pop().unwrap()
        };

        let response = method(paramter);

        println!("{} : {} : {}ms", response.url, response.code,
            response.time);
      }));
    }

    for child in children {
        let _ = child.join();
    }
}

