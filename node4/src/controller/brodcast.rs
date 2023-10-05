use reqwest::Error;

use crate::utils::metadata::Vote;

use crate::utils::ips::REMOTE_ADDRESS;

use crate::utils::metadata::NODES;

// pub struct Vote {
//     pub id : i64,
//     pub ip : String,
//     pub vote : i8
// }

pub async fn brodcast(vote : &Vote,client_add : String) -> Result<(),Error> {
    println!("Brodcating from node 4");
    let mut handles = Vec::new();
    let remote = REMOTE_ADDRESS.lock();
    for add in &*remote {
        let url1 = format!("http://{}/vote", *add);
        let url2 = format!("http://{}/running", *add);
        let json_data = serde_json::to_string(&vote).expect("Error while serializing");
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
    let mut nodes = NODES.lock();
    nodes.insert(vote.id, 0);
    if let Some(node) = nodes.get_mut(&vote.id) {
        for handle in handles {
            let res = handle.await.unwrap();
            match res {
                Ok(()) => {
                    *node = *node + 1;
                },
                Err(err) => {
                    println!("{:?}",format!("Error Message : {}",err));
                }
            }
        }
    }
    
    Ok(())
}