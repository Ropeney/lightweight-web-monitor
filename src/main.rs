use std::{thread, time};
use std::sync::{Arc};

mod website;
mod notification_queue;

fn main() {
    const THREAD_COUNT: i8 = 2;
    const WORKER_SLEEP_TIME: u64 = 2000;
    const POLL_SLEEP_TIME: u64 = 4000;

    let notifier = Arc::new(notification_queue::Notifier::new());

    {
        let notifier = notifier.clone();
        thread::spawn(move || {
            // This is where it will poll a database for results, checking if new checks
            // need to be added to the queue
            notifier.add_to_queue(website::check_site, "https://google.com");
            thread::sleep(time::Duration::from_millis(POLL_SLEEP_TIME));
            notifier.add_to_queue(website::check_site, "https://ropeney.com");
            thread::sleep(time::Duration::from_millis(POLL_SLEEP_TIME));
            notifier.add_to_queue(website::check_site, "https://google.com");
            notifier.add_to_queue(website::check_site, "https://google.com");
            thread::sleep(time::Duration::from_millis(POLL_SLEEP_TIME));
            notifier.add_to_queue(website::check_site, "https://ropeney.com");
            thread::sleep(time::Duration::from_millis(POLL_SLEEP_TIME));
            thread::sleep(time::Duration::from_millis(POLL_SLEEP_TIME));
            notifier.add_to_queue(website::check_site, "https://ropeney.com");
            thread::sleep(time::Duration::from_millis(POLL_SLEEP_TIME));
        });
    }

    let mut children = vec![];

    // Make a counter so this is correct
    for _ in 0..THREAD_COUNT {
        let notifier = notifier.clone();
        children.push(thread::spawn(move || {
            loop {
                if notifier.count() > 0 {
                    // This is so that the mutex unlocks quicker
                    let result = notifier.pop();
                    if result.is_ok() {
                        let (method, paramater) = *result.unwrap();
                        let response = method(&paramater);

                        println!("{} : {} : {}ms", response.identifier, response.state,
                            response.time);
                    } else {
                        thread::sleep(time::Duration::from_millis(WORKER_SLEEP_TIME));
                    }
                } else {
                    thread::sleep(time::Duration::from_millis(WORKER_SLEEP_TIME));
                }
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }
}

