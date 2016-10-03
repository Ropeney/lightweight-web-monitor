use std::sync::{Arc, Mutex};
use website;

pub struct Response {
    pub time: i64,
    pub identifier: String,
    pub state: String
}

pub struct Notifier {
    queue: Arc<Mutex<Vec<(fn(url: &str) -> Box<Response>, String)>>>,
}

impl Notifier {
    pub fn add_to_queue(&self, closure: fn(url: &str) -> Box<Response>, identifier: &str)  {
        let data = &self.queue.clone();
        let mut data = data.lock().unwrap();
        data.push((closure, identifier.to_string()));
    }

    pub fn new() -> Box<Notifier> {
        let new_queue = Arc::new(Mutex::new(Vec::new()));
        Box::new(Notifier {
            queue: new_queue,
        })
    }

    pub fn pop(&self) -> Result<Box<(fn(url: &str) -> Box<Response>, String)>, &'static str> {
        let data = &self.queue.clone();
        let mut data = data.lock().unwrap();
        if data.len() <= 0 {
            return Err("No threads in queue");
        }

        Ok(Box::new(data.pop().unwrap()))
    }

    // This should be a struct variable but god knows how to make it mutable
    pub fn count(&self) -> usize {
        self.queue.lock().unwrap().len()
    }
}

#[test]
pub fn notifier_add_to_queue_adds_to_queue() {
    let notifier = Arc::new(Notifier::new());
    notifier.add_to_queue(website::check_site, "https://google.com");
    assert_eq!(notifier.queue.lock().unwrap().len(), 1);
}

#[test]
pub fn notifier_count_returns_correct_number() {
    let notifier = Arc::new(Notifier::new());
    notifier.add_to_queue(website::check_site, "https://google.com");
    assert_eq!(notifier.count(), 1);
}

#[test]
pub fn notifier_pop_removes_1_result() {
    let notifier = Arc::new(Notifier::new());
    notifier.add_to_queue(website::check_site, "https://google.com");
    let _ = notifier.pop();
    assert_eq!(notifier.count(), 0);
}
