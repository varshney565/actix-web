use crate::utils::metadata::voteProposal;
use crate::utils::ips::{
    REMOTE_ADDRESS,
    THIS_IP
};

pub async fn do_post_call(proposal : &voteProposal) -> Result<(), reqwest::Error> {
    let mut handles = Vec::new();
    unsafe {
        let n = REMOTE_ADDRESS.len();
        for i in 0..n {
            if REMOTE_ADDRESS[i] == THIS_IP {
                continue;
            }
            let url = format!("http://{}/node", REMOTE_ADDRESS[i]);
            let json_data = serde_json::to_string(&proposal).expect("Error while serializing");

            // Spawn a Tokio task for each HTTP request
            let handle = tokio::spawn(async move {
                let client = reqwest::Client::new();
                let response = client
                    .post(&url)
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                    .body(json_data)
                    .send()
                    .await?;

                let status = response.status();
                let text = response.text().await?;
                println!("Status: {:?}", status);
                println!("Response: {}", text);

                Result::<(), reqwest::Error>::Ok(())
            });
            handles.push(handle);
        }

        for handle in handles {
            let res = handle.await.unwrap();
            match res {
                Ok(()) => {

                },
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
    Ok(())
}
