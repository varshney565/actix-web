use actix_web::{HttpResponse, web, HttpRequest};

use std::{env, sync::Arc};

use parking_lot::Mutex;

use futures::future::join_all;

use crate::utils::{metadata::{Proposal, PROPOSALS, NODES},ips::REMOTE_ADDRESS};

pub async fn secondry_index(proposal : web::Json<Proposal>, req : HttpRequest) -> HttpResponse {
    /*
     * insert the proposal locally.
     * */
    let new_proposal = Proposal {
        id : proposal.0.id,
        subject : proposal.0.subject.clone(),
        description : proposal.0.description.clone()
    };
    let mut proposals = PROPOSALS.lock();
    proposals.insert(proposal.0.id,new_proposal);
    drop(proposals);

    /*
     * Fetch the client-address from the header
     */
    let mut client_add : String = "".to_string();
    if let Some(caller) = req.headers().get("client-add") {
        if let Ok(caller_ip) = caller.to_str() {
            client_add = caller_ip.to_string();
        }
    }
    
    /*
     * find the ip of the current node; 
     * */
    let ip = env::var("IP").expect("Failed to Load the IP of the machine !!");
    let port = env::var("PORT").expect("Failed to fetch the Port !!");
    let this_ip = format!("{}:{}",ip,port);

    /*
    * find the active_nodes and there count;
    * */
    let mut futures = Vec::new();
    let remote = REMOTE_ADDRESS.lock();
    let active_nodes = Arc::new(Mutex::new(Vec::new()));
    for add in &*remote {
        let _add = format!("{}",add);
        if _add == this_ip {
            continue;
        }
        let url = format!("http://{}/running", add.clone());
        let active_nodes_clone = Arc::clone(&active_nodes);
        let future = async move {
            let _res = reqwest::Client::builder()
                .timeout(std::time::Duration::from_millis(100)) // Set a timeout of 5 seconds
                .build()
                .unwrap()
                .head(&url)
                .send()
                .await?;
            let mut active_nodes = active_nodes_clone.lock();
            active_nodes.push(_add.clone());
            drop(active_nodes);
            Result::<(), reqwest::Error>::Ok(())
        };
        futures.push(future);
    }
    drop(remote);
    join_all(futures).await;
    println!("PROPOSAL CAME AND VALUE CALCULATED !!");
    
    /*
    * now set the active_nodes, max_faulty_node and brodcast status locally.
    * */
    
    let _active_nodes = active_nodes.lock();
    let f = (_active_nodes.len() as i64 - 1)/3;
    let n = _active_nodes.len();
    println!("{:?}",_active_nodes);
    drop(_active_nodes);
    let mut nodes = NODES.lock();
    nodes.insert(proposal.0.id, (n as i64 + 1, f, false));
    drop(nodes);
    //wait for 1.5 second to make sure all the nodes have the upper-bound on the number of faulty nodes.
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    /*
     * now it's time to send the proposal for the validation purpose.
     * */
    let _active_nodes = active_nodes.lock();
    let mut futures = Vec::new(); 
    for _add in &*_active_nodes {
        let add = _add.clone();
        let url = format!("http://{}/signal", add.clone());
        let json_data = serde_json::to_string(&proposal).expect("Error while serializing");
        let _client_add = client_add.clone();
        let future = async move {
            let thread_id = std::thread::current().id();
            println!("Child thread ID: {:?}", thread_id);
            let _res = reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(20)) // Set a timeout of 20 seconds
                .build()
                .unwrap()
                .post(&url)
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .header("client-add",_client_add.clone())
                .body(json_data)
                .send()
                .await?;
            Result::<(), reqwest::Error>::Ok(())
        };
        futures.push(future);
    }
    drop(_active_nodes);
    join_all(futures).await;

    /*
     * Everything is perfect/
     */
    HttpResponse::Ok().json("success")
}
