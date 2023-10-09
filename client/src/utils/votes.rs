use lazy_static::lazy_static;
use std::collections::BTreeMap;
use parking_lot::Mutex;

lazy_static! {
    pub static ref VOTES : Mutex<BTreeMap<i64,BTreeMap<String,(i8,i64,i64)>>> = Mutex::new(BTreeMap::new());
}