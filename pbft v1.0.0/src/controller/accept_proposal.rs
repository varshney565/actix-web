use actix_web::{web,HttpResponse,Result};
use crate::utils::metadata::{Info,voteProposal};
use std::collections::BTreeMap;
use crate::utils::ips::THIS_IP;
use crate::controller::brodcast::do_post_call;

pub async fn index(proposal : web::Json<Info>) -> Result<HttpResponse> {
    //prepare the data as well for rest of the calls.
    let mut data = voteProposal {
        subject : proposal.0.subject,
        description : proposal.0.description,
        count : 0,
        votes : BTreeMap::new()
    };

    //also mark your own vote
    data.votes.insert(THIS_IP.to_string(), true);
    data.count += 1;
    let res = do_post_call(&data).await;
    match res {
        Ok(()) => {
            Ok(HttpResponse::Ok().json("Success"))
        },
        Err(err) => {
            let error_message = format!("Error: {:?}", err);
            let error_response = HttpResponse::InternalServerError().json(error_message);
            Ok(error_response)
        }
    }
}
