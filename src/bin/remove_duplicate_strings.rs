use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};

fn remove_duplicate_lines(file_path: &str) -> io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    let mut seen = HashSet::new();
    let mut unique_lines = Vec::new();

    for line in content.lines() {
        if seen.insert(line.to_string()) {
            unique_lines.push(line);
        }
    }

    let mut file = fs::File::create(file_path)?;
    for line in unique_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn main() {
    let file_path = "strings.txt"; // change if needed
    if let Err(e) = remove_duplicate_lines(file_path) {
        eprintln!("Error: {}", e);
    }
}
