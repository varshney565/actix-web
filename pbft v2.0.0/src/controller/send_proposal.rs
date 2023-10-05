use actix_web::web;
use crate::utils::{metadata::Proposal, ips::REMOTE_ADDRESS};
use std::env;

pub async fn send_proposal(proposal : &web::Json<Proposal>,client_add : String) -> Result<(),reqwest::Error> {
    let ip = env::var("IP").expect("Failed to Load the IP of the machine !!");
    let port = env::var("PORT").expect("Failed to fetch the Port !!");
    let this_ip = format!("{}:{}",ip,port);
    let mut handles = Vec::new();
    let remote = REMOTE_ADDRESS.lock();
    for add in &*remote {
        if *add == this_ip {
            continue;
        }
        let url1 = format!("http://{}/node", *add);
        let url2 = format!("http://{}/running", *add);
        let json_data = serde_json::to_string(&proposal).expect("Error while serializing");
        
        // Spawn a Tokio task for each HTTP request
        let client_address = client_add.clone();
        let handle = tokio::spawn(async move {
            let _res = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(1)) // Set a timeout of 20 seconds
                .build()
                .unwrap()
                .head(&url2)
                .send()
                .await?;
            let _res = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(7)) // Set a timeout of 20 seconds
                .build()
                .unwrap()
                .post(&url1)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .header("client-add", client_address)
                .body(json_data)
                .send()
                .await?;

            Result::<(), reqwest::Error>::Ok(())
        });
        handles.push(handle);
    }
    for handle in handles {
        let res = handle.await.unwrap();
        if let Err(err) = res {
            println!("{:?}",format!("Error Message : {}",err));
        }
    }
    Ok(())
}