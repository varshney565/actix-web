use crate::utils::remote_address::IP_QUEUE;
pub async fn logic() -> &'static str {
    let mut queue = IP_QUEUE.lock();
    let ip_address = *queue.front().unwrap();
    queue.pop_front();
    queue.push_back(ip_address);
    drop(queue);
    ip_address
}