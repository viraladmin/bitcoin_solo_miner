use serde::Deserialize;
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct BlockTemplateTx {
    pub data: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BlockTemplate {
    pub version: i32,
    pub previousblockhash: String,
    pub height: u64,
    pub bits: String,
    pub curtime: u64,
    pub coinbasevalue: u64,
    pub transactions: Vec<BlockTemplateTx>,
}


pub async fn fetch_block_template(
    client: &Client,
    rpc_url: &str,
    rpc_user: &str,
    rpc_pass: &str,
) -> Result<BlockTemplate, Box<dyn Error + Send + Sync>> {
    let payload = json!({
        "jsonrpc": "1.0",
        "id": "rustminer",
        "method": "getblocktemplate",
        "params": [{ "rules": ["segwit"] }]
    });

    let res = client
        .post(rpc_url)
        .basic_auth(rpc_user, Some(rpc_pass))
        .json(&payload)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let template: BlockTemplate = serde_json::from_value(res["result"].clone())?;
    Ok(template)
}
