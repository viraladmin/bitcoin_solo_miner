use reqwest::Client;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize)]
struct Facts {
    text: String,
}

async fn fetch_and_save_facts() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://uselessfacts.jsph.pl/api/v2/facts/random";

    loop {
        let resp = client
            .get(url)
            .send()
            .await?
            .error_for_status()? // fail if non-200
            .json::<Facts>() // 👈 single object, not Vec
            .await?;

        if resp.text.len() < 90 {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("strings.txt")?;
            writeln!(file, "{}", resp.text)?;
        }

        sleep(Duration::from_millis(600)).await;
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = fetch_and_save_facts().await {
        eprintln!("Error: {}", e);
    }
}
