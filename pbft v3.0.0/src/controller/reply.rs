use reqwest::Error;

use crate::utils::metadata::Vote;

pub async fn reply(vote : &Vote,client_add : String) -> Result<(),Error>{
    let url = format!("http://{}/reply", client_add.to_string());
    let json_data = serde_json::to_string(&vote).expect("Error while serializing");
    let _res = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(7)) // Set a timeout of 20 seconds
            .build()
            .unwrap()
            .post(&url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(json_data)
            .send()
            .await?;
    Ok(())
}