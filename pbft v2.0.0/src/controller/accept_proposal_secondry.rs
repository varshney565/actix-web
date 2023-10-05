use actix_web::{HttpResponse, web, HttpRequest};

use crate::utils::metadata::Vote;

use crate::utils::metadata::Proposal;

use crate::controller::brodcast::brodcast;

use crate::controller::verification::verify;

use crate::controller::reply::reply;

use std::env;

pub async fn secondry_index(proposal : web::Json<Proposal>, req : HttpRequest) -> HttpResponse {
    let mut client_add : String = "".to_string();
    if let Some(caller) = req.headers().get("client-add") {
        if let Ok(caller_ip) = caller.to_str() {
            client_add = caller_ip.to_string();
        }
    }
    let ip = env::var("IP").expect("Failed to Load the IP of the machine !!");
    let port = env::var("PORT").expect("Failed to fetch the Port !!");
    let this_ip = format!("{}:{}",ip,port);
    let my_vote = Vote {
        id : proposal.0.id,
        ip : this_ip.clone(),
        vote : 1
    };
    let res = brodcast(&my_vote,client_add.clone()).await;
    match res {
        Ok(()) => {
            let response = verify(proposal.0.id).await;
            let _reply = Vote {
                id : proposal.0.id,
                ip : this_ip.clone(),
                vote : response
            };
            let res = reply(&_reply,client_add).await;
            match res {
                Ok(()) => {
                    HttpResponse::Ok().json("success!")
                },
                Err(err) => {
                    let error_message = format!("Error: {:?}", err);
                    let error_response = HttpResponse::InternalServerError().json(error_message);
                    error_response
                }
            }
        },
        Err(err) => {
            let error_message = format!("Error: {:?}", err);
            let error_response = HttpResponse::InternalServerError().json(error_message);
            error_response
        }
    }
}
