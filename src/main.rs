mod block_template;
use block_template::{ BlockTemplate, fetch_block_template };

mod block_updater;
use block_updater::update_block;

mod merkle;
mod mine;

mod utils;
use utils::load_sentences_from_file;


use dotenvy::dotenv;
use mine::mine_loop;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use utils::{build_full_block, submit_block};

pub static mut SENTENCES: &[String] = &[];
pub static GLOBAL_TEMPLATE: Lazy<RwLock<Option<(Arc<BlockTemplate>, bool)>>> = Lazy::new(|| RwLock::new(None));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    let num_tasks: i64 = dotenvy::var("NUM_TASKS").ok().and_then(|s| s.parse().ok()).unwrap_or(1);
    let rpc_url = dotenvy::var("RPC_URL").expect("RPC_URL must be set");
    let rpc_user = dotenvy::var("RPC_USER").expect("RPC_USER must be set");
    let rpc_pass = dotenvy::var("RPC_PASS").expect("RPC_PASS must be set");
    let payout_address = dotenvy::var("PAYOUT_ADDRESS").expect("PAYOUT_ADDRESS must be set");
    let strings_file = dotenvy::var("STRINGS_FILE").expect("STRINGS_FILE must be set");

    let client = Arc::new(Client::new());

    update_block(client.clone(), rpc_url.to_string(), rpc_user.to_string(), rpc_pass.to_string());
    load_sentences_from_file(&strings_file);



#[cfg(windows)]
{
    use std::thread;
    use tokio::runtime::Runtime;

    for _ in 0..num_tasks {
        let payout_address = payout_address.clone();
        let client = Arc::clone(&client);
        let rpc_url = rpc_url.clone();
        let rpc_user = rpc_user.clone();
        let rpc_pass = rpc_pass.clone();

        thread::spawn(move || {
            let rt = Runtime::new().unwrap();
            loop {
                if let Some(mined_block) = rt.block_on(mine_loop(&payout_address)).unwrap() {
                    let block_bytes = build_full_block(mined_block.header, mined_block.transactions);
                    rt.block_on(submit_block(&client, &rpc_url, &rpc_user, &rpc_pass, block_bytes)).unwrap();
                }
            }
        });
    }
}

#[cfg(not(windows))]
{
    // Spawn dedicated mining workers
    for _ in 0..num_tasks {
        let client = Arc::clone(&client);
        let rpc_url_clone = rpc_url.clone();
        let rpc_user_clone = rpc_url.clone();
        let rpc_pass_clone = rpc_pass.clone();
        let payout_address_clone = payout_address.clone();
        tokio::spawn({
            async move {
                loop {
                    let result: Result<(), Box<dyn std::error::Error + Send + Sync>> = async {
                        if let Some(mined_block) = mine_loop(&payout_address_clone).await? {
                            let block_bytes = build_full_block(mined_block.header, mined_block.transactions);
                            submit_block(&client, &rpc_url_clone, &rpc_user_clone, &rpc_pass_clone, block_bytes).await?;
                        }
                        Ok(())
                    }
                    .await;

                    if let Err(e) = result {
                        eprintln!("Mining task failed: {e}");
                    }
                }
            }
        });
    }
}
    // Keep main alive
    futures::future::pending::<()>().await;
    Ok(())
}
