use serde::{Deserialize,Serialize};
use std::collections::BTreeMap;
#[derive(Serialize,Deserialize,Debug)]
pub struct Info {
    pub subject : String,
    pub description : String
}

#[derive(Serialize,Deserialize,Debug)]
pub struct voteProposal {
    pub subject : String,
    pub description : String,
    pub count : u32,
    pub votes : BTreeMap<String,bool>
}
