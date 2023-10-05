use actix_web::{web, HttpResponse, HttpRequest};

use crate::utils::metadata::Proposal;

use crate::utils::metadata::Vote;

use crate::controller::send_proposal::send_proposal;

use crate::controller::brodcast::brodcast;

use crate::controller::verification::verify;

use crate::controller::reply::reply;

use std::env;

pub async fn index(proposal : web::Json<Proposal>, req : HttpRequest) -> HttpResponse {
    let mut client_add : String = "".to_string();
    if let Some(caller) = req.headers().get("client-add") {
        if let Ok(caller_ip) = caller.to_str() {
            client_add = caller_ip.to_string();
        }
    }

    //pre-prepare phase 
    let res = send_proposal(&proposal,client_add.clone()).await;
    match res {
        Ok(()) => {
            //prepare phase
            let ip = env::var("IP").expect("Failed to Load the IP of the machine !!");
            let port = env::var("PORT").expect("Failed to fetch the Port !!");
            let this_ip = format!("{}:{}",ip,port);
            let my_vote = Vote {
                id : proposal.0.id,
                ip : this_ip.clone(),
                vote : 1
            };
            
            let res = brodcast(&my_vote,client_add.clone()).await;
            // tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
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
        },
        Err(err) => {
            HttpResponse::InternalServerError().json(format!("ErrorMessage : {}",err))
        }
    }
}
