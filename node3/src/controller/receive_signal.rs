use actix_web::{HttpResponse, HttpRequest};

use crate::utils::metadata::Vote;

use crate::controller::brodcast::brodcast;

use crate::controller::verification::verify;

use crate::controller::reply::reply;

use std::env;

pub async fn receive_signal(req : HttpRequest) -> HttpResponse {
    let mut proposal_id : i64 = 0;
    if let Some(id) = req.headers().get("id") {
        if let Ok(_id) = id.to_str() {
            proposal_id = _id.parse::<i64>().unwrap();
        }
    }
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
        id : proposal_id,
        ip : this_ip.clone(),
        vote : 0
    };
    let res = brodcast(&my_vote,client_add.clone()).await;
    match res {
        Ok(()) => {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let response = verify(proposal_id).await;
            let _reply = Vote {
                id : proposal_id,
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