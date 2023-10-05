use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct Vote {
    pub id : i64,
    pub ip : String,
    pub vote : i8
}