use lazy_static::lazy_static;
use std::collections::VecDeque;
use parking_lot::Mutex;

lazy_static! {
    pub static ref IP_QUEUE : Mutex<VecDeque<&'static str>> = Mutex::new(VecDeque::new());
}

pub fn add_node(node : &'static str) {
    let mut queue = IP_QUEUE.lock();
    queue.push_back(node);
    drop(queue);
}