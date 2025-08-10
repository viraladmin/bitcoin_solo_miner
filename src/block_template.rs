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

    if let Some(err) = res.get("error") {
        if !err.is_null() {
            // Log the whole error so you can see code/message
            return Err(format!("getblocktemplate error: {}", err).into());
        }
    }

    let result = res
        .get("result")
        .ok_or("getblocktemplate: missing 'result'")?;

    if result.is_null() {
        // Defensive: null without an error object (rare, but handle it).
        return Err("getblocktemplate returned null result".into());
    }

    let template: BlockTemplate = serde_json::from_value(result.clone())?;
    Ok(template)
}
