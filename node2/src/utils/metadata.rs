use serde::{Deserialize,Serialize};
use std::collections::BTreeMap;
use lazy_static::lazy_static;
use parking_lot::Mutex;
#[derive(Serialize,Deserialize,Debug)]
pub struct Proposal {
    pub id : i64,
    pub subject : String,
    pub description : String
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Vote {
    pub id : i64,
    pub ip : String,
    pub vote : i8
}

impl PartialEq for Proposal {
    fn eq(&self, other: &Self) -> bool {
        // Implement your custom comparison logic here
        self.id == other.id && self.subject == other.subject && self.description == other.description
    }
}

impl Eq for Proposal {}


lazy_static! {
    pub static ref VOTES : Mutex<BTreeMap<i64,BTreeMap<String,i8>>> = Mutex::new(BTreeMap::new());
    //ProposalId -> (Active Nodes, faultyNodes(upper_bound), brodcastedOrNot)
    pub static ref NODES : Mutex<BTreeMap<i64,(i64,i64,bool)>> = Mutex::new(BTreeMap::new()); 
    pub static ref ACCEPTER : Mutex<BTreeMap<i64,i64>> = Mutex::new(BTreeMap::new());
    pub static ref PROPOSALS : Mutex<BTreeMap<i64,Proposal>> = Mutex::new(BTreeMap::new());
}