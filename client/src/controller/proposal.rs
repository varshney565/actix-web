use reqwest;
use serde_json;
use actix_web::{web, HttpResponse};
use std::env;
use crate::controller::verification::verification;
use crate::controller::ip_selector::logic; 
use crate::utils::proposal_structure::Proposal;

pub async fn index(proposal : web::Json<Proposal>) -> HttpResponse {
    //fetching env variables.
    let this_ip = env::var("IP").expect("Failed to Load the IP of the machine !!");
    let port = env::var("PORT").expect("Failed to fetch the Port !!");
    let mut ip_address;
    //choosing the primary.
    loop {
        ip_address = logic().await;
        let res = reqwest::Client::new().head(format!("http://{}/running",ip_address)).send().await;
        match res {
            Ok(res) => {
                if res.status() == 200 {
                    break;
                }
            },
            Err(err) => {
                println!("{:}",err);
                continue;
            }
        }
    }
    //url to call on.
    let url = format!("http://{}/proposal",ip_address);
    //client to call on.
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30)) // Set a timeout of 20 seconds
        .build()
        .unwrap();
    //preparing the request body.
    let json_data = serde_json::to_string(&proposal).expect("Error while serializing !");
    let response = client
        .post(&url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header("client-add",format!("{}:{}",this_ip,port))
        .body(json_data)
        .send()
        .await;
    match response {
        Ok(res) => {
            let status = res.status();
            if status != 200 {
                let text = res.text().await.unwrap();
                HttpResponse::build(status).json(text)
            }else {
                let result = verification(proposal.0.id);
                HttpResponse::build(status).json(result)
            }
        },
        Err(err) => {   
            let error_message = format!("Error: {:?}", err);
            let error_response = HttpResponse::InternalServerError().json(error_message);
            error_response
        }
    }
}
