use actix_web::{web, HttpResponse, HttpRequest};

use crate::utils::metadata::PROPOSALS;

use crate::utils::metadata::Proposal;

use crate::controller::send_proposal::send_proposal;

use super::send_signal::signal;

pub async fn index(proposal : web::Json<Proposal>, req : HttpRequest) -> HttpResponse {
    let mut client_add : String = "".to_string();
    if let Some(caller) = req.headers().get("client-add") {
        if let Ok(caller_ip) = caller.to_str() {
            client_add = caller_ip.to_string();
        }
    }

    //pre-prepare phase 
    let new_proposal = Proposal {
        id : proposal.0.id,
        subject : proposal.0.subject.clone(),
        description : proposal.0.description.clone()
    };
    let mut proposals = PROPOSALS.lock();
    proposals.insert(proposal.0.id,new_proposal);
    drop(proposals);
    let res = send_proposal(&proposal).await;
    match res {
        Ok(()) => {
            //prepare phase
            match res {
                Ok(()) => {
                    let res = signal(proposal.0.id, client_add.clone()).await;
                    match res {
                        Ok(()) => {
                            HttpResponse::Ok().json("success")
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
