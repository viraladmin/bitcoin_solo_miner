use crate::addresses::remove_used_address;
use crate::{GLOBAL_TEMPLATE, PAYOUT_ADDRESS, SENTENCES};
use crate::merkle::{ compute_root_from_branch, LAST_TXS, };
use crate::utils::{
    build_coinbase_tx, bits_to_target, hash_meets_target, serialize_header_to_array, 
};
use bitcoin::{Address, Network};
use std::str::FromStr;
use std::sync::Arc;
use bitcoin::hashes::Hash;
use fastrand;

pub struct MinedBlock {
    pub header: bitcoin::block::Header,
    pub transactions: Vec<Arc<bitcoin::Transaction>>,
}

pub fn pick_random_sentence() -> &'static str {
    unsafe {
        if (*SENTENCES).is_empty() {
            "default extra nonce"
        } else {
            &(*SENTENCES)[fastrand::usize(..(*SENTENCES).len())]
        }
    }
}

pub async fn mine_loop() -> Result<Option<MinedBlock>, Box<dyn std::error::Error + Send + Sync>> {
    let payout_address = {
        let lock = PAYOUT_ADDRESS.read().await;
        match &*lock {
            Some(addr) => addr.clone(),
            None => return Ok(None),
        }
    };

    // Get the current block template (read-only)
    let template = {
        let guard = GLOBAL_TEMPLATE.read().await;
        match &*guard {
            Some((t, _)) => Arc::clone(t),
            None => return Ok(None),
        }
    };

    // Get the cached transactions & merkle branches (read-only)
    let cached = {
        let guard = LAST_TXS.read().await;
        match &*guard {
           Some(c) => Arc::clone(c),
           None => return Ok(None),
        }
    };

    // Pick sentence for extra nonce
    let sentence = pick_random_sentence();

    // Parse payout address
    let address: Address = payout_address
        .parse::<bitcoin::address::Address<bitcoin::address::NetworkUnchecked>>()?
        .require_network(Network::Bitcoin)?;

    let bits = u32::from_str_radix(&template.bits, 16)?;
    let target = bits_to_target(bits);

    // Step 1: Dummy coinbase (no witness commitment)
    let dummy_coinbase = build_coinbase_tx(
        template.height as u32,
        sentence.as_bytes(),
        template.coinbasevalue,
        &address,
        None,
    )?;

    // Step 2: Compute witness root from dummy coinbase
    let witness_root = compute_root_from_branch(
        dummy_coinbase.wtxid().to_byte_array(),
        &cached.witness_branch,
        0,
    );

    // Step 3: Final coinbase with witness commitment
    let mut txs = cached.txs.clone();
    let coinbase_tx = build_coinbase_tx(
        template.height as u32,
        sentence.as_bytes(),
        template.coinbasevalue,
        &address,
        Some(witness_root),
    )?;
    let coinbase_txid = coinbase_tx.txid().to_byte_array();
    txs[0] = Arc::new(coinbase_tx);

    // Step 4: Compute normal merkle root
    let merkle_root = compute_root_from_branch(
        coinbase_txid,
        &cached.merkle_branch,
        0,
    );

    // Step 5: Build block header
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as u32;
    let block_time = now.max(template.curtime as u32);

    let num = fastrand::u32(..);

    let header = bitcoin::block::Header {
        version: bitcoin::block::Version::from_consensus(template.version),
        prev_blockhash: bitcoin::BlockHash::from_str(&template.previousblockhash)?,
        merkle_root: bitcoin::TxMerkleNode::from_slice(&merkle_root)?,
        time: block_time,
        bits: bitcoin::CompactTarget::from_consensus(bits),
        nonce: num,
    };

    // Step 6: Hash check
    let header_bytes = serialize_header_to_array(&header);
    if hash_meets_target(&header_bytes, &target) {
        println!("✅ Block found!");
        remove_used_address(&payout_address).await?;
        return Ok(Some(MinedBlock {
            header,
            transactions: txs,
        }));
    }

    Ok(None)
}
