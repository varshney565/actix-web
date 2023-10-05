use actix_web::{HttpResponse, web};

use crate::utils::metadata::{Proposal, PROPOSALS};

pub async fn secondry_index(proposal : web::Json<Proposal>) -> HttpResponse {
    let mut proposals = PROPOSALS.lock();
    proposals.insert(proposal.0.id, proposal.0);
    drop(proposals);
    HttpResponse::Ok().json("success")
}
