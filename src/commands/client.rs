/// Talk to my local LLM sever

use crate::Error;
use reqwest;

#[allow(unused)]
pub async fn prompt_msg(msg:&String) -> Result<(),Error>{
    let json = &serde_json::json!({
        "promptWord":msg,
        "top_p":0.2,
        "temperature":0.7,
    });

    let client = reqwest::Client::new();
    let _: reqwest::Response = client.post("http://127.0.0.1:8088/prompt")
        .json(json)
        .send()
        .await?;
    Ok(())
}