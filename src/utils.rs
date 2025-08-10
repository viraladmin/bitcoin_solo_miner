use bitcoin::{
    block::Header as BlockHeader, consensus::Encodable, hashes::{sha256d, Hash}, Amount, OutPoint,
    Sequence, Transaction, TxIn, TxOut, Witness, absolute::LockTime, address::Address, ScriptBuf,
    Block
};
use crate::SENTENCES;
use std::sync::Arc;

pub fn build_coinbase_tx(
    height: u32,
    extra: &[u8],
    reward: u64,
    address: &Address,
    witness_commitment: Option<[u8; 32]>, // NEW
) -> Result<Transaction, Box<dyn std::error::Error + Send + Sync>> {
    use bitcoin::blockdata::script::{Builder, PushBytesBuf};

    // Build input script
    let mut push = PushBytesBuf::new();
    push.extend_from_slice(extra)?;
    let input_script = Builder::new()
        .push_int(height as i64)
        .push_slice(&push)
        .into_script();

    let input = TxIn {
        previous_output: OutPoint::null(),
        script_sig: input_script,
        sequence: Sequence::MAX,
        witness: Witness::from(Vec::<Vec<u8>>::new()),
    };

    // Primary miner payout
    let output = TxOut {
        value: Amount::from_sat(reward),
        script_pubkey: address.payload().script_pubkey(),
    };

    let mut outputs = vec![output];

    // If SegWit commitment is provided, append it
    if let Some(commitment) = witness_commitment {
        let mut data = Vec::with_capacity(38);
        data.push(0x6a); // OP_RETURN
        data.push(0x24); // Push 36 bytes
        data.extend_from_slice(&[0xaa, 0x21, 0xa9, 0xed]); // Commitment tag
        data.extend_from_slice(&commitment); // Witness merkle root
        outputs.push(TxOut {
            value: Amount::from_sat(0),
            script_pubkey: ScriptBuf::from_bytes(data.into()),
        });
    }

    Ok(Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![input],
        output: outputs,
    })
}


pub fn load_sentences_from_file(path: &str) {
    let content = std::fs::read_to_string(path).expect("Cannot read sentence file");
    let lines: Vec<String> = content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    unsafe {
        SENTENCES = Box::leak(lines.into_boxed_slice());
    }
}


pub fn bits_to_target(bits: u32) -> [u8; 32] {
    let exponent = ((bits >> 24) & 0xff) as usize;
    let mut mantissa = bits & 0x007fffff;

    let mut target = [0u8; 32];
    if exponent <= 3 {
        mantissa >>= 8 * (3 - exponent);
        target[31] = (mantissa & 0xff) as u8;
        if exponent > 1 { target[30] = ((mantissa >> 8) & 0xff) as u8; }
        if exponent > 2 { target[29] = ((mantissa >> 16) & 0xff) as u8; }
    } else {
        let start = 32 - exponent;
        if start < 32 { target[start] = ((mantissa >> 16) & 0xff) as u8; }
        if start + 1 < 32 { target[start + 1] = ((mantissa >> 8) & 0xff) as u8; }
        if start + 2 < 32 { target[start + 2] = (mantissa & 0xff) as u8; }
    }
    target
}

pub fn serialize_header_to_array(header: &bitcoin::block::Header) -> [u8; 80] {
    let mut buf = [0u8; 80];
    let mut i = 0;

    buf[i..i+4].copy_from_slice(&header.version.to_consensus().to_le_bytes()); i += 4;
    buf[i..i+32].copy_from_slice(&header.prev_blockhash.as_byte_array()[..]); i += 32;
    buf[i..i+32].copy_from_slice(&header.merkle_root.as_byte_array()[..]); i += 32;
    buf[i..i+4].copy_from_slice(&header.time.to_le_bytes()); i += 4;
    buf[i..i+4].copy_from_slice(&header.bits.to_consensus().to_le_bytes()); i += 4;
    buf[i..i+4].copy_from_slice(&header.nonce.to_le_bytes());

    buf
}

pub fn hash_meets_target(header: &[u8], target: &[u8; 32]) -> bool {
    let hash = sha256d::Hash::hash(header);
    let hash_bytes = hash.to_byte_array();

    for i in 0..32 {
        if hash_bytes[i] < target[i] {
            return true;
        } else if hash_bytes[i] > target[i] {
            return false;
        }
    }
    true
}

pub fn build_full_block(
    header: BlockHeader,
    txs: Vec<Arc<Transaction>>,
) -> Vec<u8> {
    let block = Block {
        header,
        txdata: txs.into_iter().map(|tx| (*tx).clone()).collect(),
    };
    let mut buf = Vec::new();
    block.consensus_encode(&mut buf).unwrap();
    buf
}

pub async fn submit_block(
    client: &reqwest::Client,
    rpc_url: &str,
    rpc_user: &str,
    rpc_pass: &str,
    block_bytes: Vec<u8>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let block_hex = hex::encode(block_bytes);
    let payload = serde_json::json!({
        "jsonrpc": "1.0",
        "id": "rustminer",
        "method": "submitblock",
        "params": [block_hex]
    });

    let res = client
        .post(rpc_url)
        .basic_auth(rpc_user, Some(rpc_pass))
        .json(&payload)
        .send()
        .await?;

    println!("Submit response: {}", res.text().await?);
    Ok(())
}

