use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Reply {
    pub from : String,
    pub vote : i8,
    pub id : i64,
    pub f : i64,
    pub total : i64
}