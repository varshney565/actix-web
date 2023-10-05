use std::collections::BTreeMap;

use actix_web::{web, HttpResponse};
use crate::utils::votes::VOTES;
use crate::utils::reply_structure::Vote;
pub async fn reply(vote : web::Json<Vote>) -> HttpResponse {
    let mut votes = VOTES.lock();
    let id = vote.0.id;
    if let Some(existing_map) = votes.get_mut(&id) {
        existing_map.insert(vote.0.ip,vote.0.vote);
    }else {
        let mut new_map = BTreeMap::new();
        new_map.insert(vote.0.ip, vote.0.vote);
        votes.insert(id, new_map);
    }
    drop(votes);
    HttpResponse::Ok().json("success!")
}