use actix_web::{web,HttpResponse,Result};
use crate::utils::ips::{THIS_IP,REMOTE_ADDRESS};
use crate::utils::metadata::voteProposal;
use crate::controller::brodcast::do_post_call;

pub async fn vote(data : web::Json<voteProposal>) -> Result<HttpResponse> {
    //check if the current node has voted or not
    //if yes simply return from here
    //else
    //mark his own entry

    let mut data = data.0;
    if data.votes.contains_key(THIS_IP){
        return Ok(HttpResponse::AlreadyReported().json("Success"))
    }

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
