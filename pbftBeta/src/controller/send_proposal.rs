use actix_web::web;

use crate::utils::metadata::PROPOSALS;

use crate::utils::{metadata::{Proposal, NODES}, ips::REMOTE_ADDRESS};

use std::{env, sync::Arc};

use parking_lot::Mutex;

use futures::future::join_all;

pub async fn send_proposal(proposal : &web::Json<Proposal>, client_add : String) -> Result<(),reqwest::Error> {
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

    /*
     * now set the active_nodes, max_faulty_node and brodcast status locally.
     * */
    
    let _active_nodes = active_nodes.lock();
    let n = _active_nodes.len();
    let f = (n as i64 )/3;
    println!("{},{}",n,f);
    println!("{:?}",_active_nodes);
    drop(_active_nodes);
    let mut nodes = NODES.lock();
    nodes.insert(proposal.0.id, (n as i64 + 1, f, false));
    drop(nodes);
    
    /*
     * now iterate on all the active nodes and iterate on them  
     * */
    let _active_nodes = active_nodes.lock();
    let mut futures = Vec::new(); 
    for _add in &*_active_nodes {
        let add = _add.clone();
        let url = format!("http://{}/node", add.clone());
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
     * Everything is perfect.
     */
    Ok(())
}