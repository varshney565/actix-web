use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Proposal {
    pub id : i64,
    pub subject : String,
    pub description : String
}