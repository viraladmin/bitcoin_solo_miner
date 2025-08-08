use crate::PAYOUT_ADDRESS;
use std::time::Duration;
use tokio::time::sleep;

pub fn rotate_payout_address() -> () {
    tokio::spawn({
        async move {
            loop {
                let path = dotenvy::var("PAYOUT_ADDRESSES").expect("PAYOUT_ADDRESSES must be set");
                sleep(Duration::from_secs(30)).await;
 
                match std::fs::read_to_string(path) {
                   Ok(content) => {
                        let lines: Vec<_> = content
                            .lines()
                            .map(str::trim)
                            .filter(|l| !l.is_empty())
                            .map(str::to_string)
                            .collect();
  
                        if lines.is_empty() {
                            eprintln!("No addresses in file");
                            continue;
                        }

                        let mut new_address;
                        let current = PAYOUT_ADDRESS.read().await.clone();

                        // Select a new one that’s different
                        loop {
                            new_address = lines[fastrand::usize(..lines.len())].clone();
                            if Some(&new_address) != current.as_ref() || lines.len() == 1 {
                                break;
                            }
                        }

                        let mut lock = PAYOUT_ADDRESS.write().await;
                        *lock = Some(new_address.clone());
                    }
                    Err(e) => {
                        eprintln!("Failed to read payout addresses: {e}");
                    }
                }
            }
        }
    });
}

pub async fn remove_used_address(used: &str) -> std::io::Result<()> {
    let addresses_file = dotenvy::var("PAYOUT_ADDRESSES").expect("PAYOUT_ADDRESS must be set");
    let contents = std::fs::read_to_string(&addresses_file)?;
    let used = used.trim(); // ← Trim once here

    let filtered: Vec<&str> = contents
        .lines()
        .filter(|line| line.trim() != used)
        .collect();

    std::fs::write(addresses_file, filtered.join("\n"))?;
    Ok(())
}
