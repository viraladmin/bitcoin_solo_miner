use bitcoin::hashes::Hash;
use bitcoin::Transaction;
use crate::block_template::BlockTemplate;
use crate::fetch_block_template;
use crate::GLOBAL_TEMPLATE;
use crate::merkle::{ CachedTemplate, LAST_TXS, precompute_merkle_branch };
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub fn update_block(client: Arc<Client>, rpc_url: String, rpc_user: String, rpc_pass: String) -> () {
    // Spawn block template updater
    tokio::spawn({
        let client = Arc::clone(&client);
        async move {
            loop {
                let existing_hash = {
                    let guard = GLOBAL_TEMPLATE.read().await;
                    guard.as_ref().map(|(t, _)| t.previousblockhash.clone())
                };

                match fetch_block_template(&client, &rpc_url, &rpc_user, &rpc_pass).await {
                    Ok(template) => {
                        let template_arc = Arc::new(template);

                        if existing_hash.as_deref() != Some(&template_arc.previousblockhash) {
                            let mut guard = GLOBAL_TEMPLATE.write().await;
                            *guard = Some((Arc::clone(&template_arc), true));
                            if let Err(e) = update_tx_cache_once(Arc::clone(&template_arc)).await {
                                eprintln!("Error updating transaction cache: {e}");
                            }
                        }
                    }
                    Err(e) => eprintln!("Error updating block template: {e}"),
                }

                sleep(Duration::from_secs(45)).await;
            }
        }
    });
}

pub async fn update_tx_cache_once(
    template: Arc<BlockTemplate>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Rebuilding transaction cache and merkle branches...");

    let mut txs_decoded: Vec<Arc<Transaction>> = Vec::with_capacity(template.transactions.len() + 1);

    txs_decoded.push(Arc::new(Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![],
        output: vec![],
    }));

    for tx in &template.transactions {
        let raw = hex::decode(&tx.data)?;
        let tx: bitcoin::Transaction = bitcoin::consensus::deserialize(&raw)?;
        txs_decoded.push(Arc::new(tx));
    }

    let txids: Vec<[u8; 32]> = txs_decoded[1..]
        .iter()
        .map(|t| t.txid().to_byte_array())
        .collect();
    let wtxids: Vec<[u8; 32]> = txs_decoded[1..]
        .iter()
        .map(|t| t.wtxid().to_byte_array())
        .collect();

    let merkle_branch = precompute_merkle_branch(
        std::iter::once([0u8; 32]).chain(txids.into_iter()).collect(),
        0,
    );
    let witness_branch = precompute_merkle_branch(
        std::iter::once([0u8; 32]).chain(wtxids.into_iter()).collect(),
        0,
    );

    *LAST_TXS.write().await = Some(Arc::new(CachedTemplate {
        txs: txs_decoded,
        merkle_branch,
        witness_branch,
    }));

    Ok(())
}
