/// Talk to my local LLM sever
use crate::Error;

use reqwest;
use serde::{Deserialize, Serialize};


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct PromptData{
    ouput:String,
    promptWord:String,
    temperature:f32,
    top_p:f32,

}
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct PromptResult{
    data:PromptData,
    status:String,
}


#[allow(unused)]
pub async fn prompt_msg(msg:&String) -> Result<String,Error>{
    let json = &serde_json::json!({
        "promptWord":msg,
        "top_p":0.2,
        "temperature":0.7,
    });

    let client = reqwest::Client::new();
    let res= client.post("http://127.0.0.1:8088/prompt")
        .json(json)
        .send()
        .await?
        .text()
        .await?;
    let res_json:PromptResult = serde_json::from_str(res.as_str()).unwrap();

    Ok(res_json.data.ouput)
}