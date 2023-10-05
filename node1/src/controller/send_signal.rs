use crate::utils::ips::REMOTE_ADDRESS;
use reqwest::Error;
pub async fn signal(id : i64, client_add : String) -> Result<(),Error> {
    let mut handles = Vec::new();
    let remote = REMOTE_ADDRESS.lock();
    for add in &*remote {
        let url1 = format!("http://{}/signal", *add);
        let url2 = format!("http://{}/running", *add);
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
                .header("id", id)
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