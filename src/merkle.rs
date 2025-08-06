use bitcoin::hashes::{sha256d, Hash};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct CachedTemplate {
    pub txs: Vec<Arc<bitcoin::Transaction>>,
    pub merkle_branch: Vec<[u8; 32]>,
    pub witness_branch: Vec<[u8; 32]>,
}

pub static LAST_TXS: Lazy<RwLock<Option<Arc<CachedTemplate>>>> = Lazy::new(|| RwLock::new(None));


pub fn precompute_merkle_branch(mut hashes: Vec<[u8; 32]>, mut index: usize) -> Vec<[u8; 32]> {
    let mut branch = Vec::new();
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            // duplicate last hash if odd number of entries
            hashes.push(*hashes.last().unwrap());
        }
        let sibling_index = if index % 2 == 0 { index + 1 } else { index - 1 };
        branch.push(hashes[sibling_index]);
        index /= 2;
        // combine in pairs for next level
        hashes = hashes.chunks(2)
            .map(|pair| {
                let mut concat = [0u8; 64];
                concat[..32].copy_from_slice(&pair[0]);
                concat[32..].copy_from_slice(&pair[1]);
                sha256d::Hash::hash(&concat).to_byte_array()
            })
            .collect();
    }
    branch
}

pub fn compute_root_from_branch(mut hash: [u8; 32], branch: &[[u8; 32]], mut index: usize) -> [u8; 32] {
    for sibling in branch {
        let mut concat = [0u8; 64];
        if index % 2 == 0 {
            concat[..32].copy_from_slice(&hash);
            concat[32..].copy_from_slice(sibling);
        } else {
            concat[..32].copy_from_slice(sibling);
            concat[32..].copy_from_slice(&hash);
        }
        hash = sha256d::Hash::hash(&concat).to_byte_array();
        index /= 2;
    }
    hash
}
