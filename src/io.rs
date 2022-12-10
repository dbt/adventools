pub use anyhow::Result;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_lines(path: &str) -> Result<Vec<String>> {
    let f = File::open(path)?;
    let lines: io::Result<Vec<String>> = BufReader::new(f).lines().collect();
    Ok(lines?
        .into_iter()
        .map(|s| s.trim_end().to_string())
        .collect())
}

pub fn split_str(buf: &str) -> Vec<String> {
    buf.split('\n').map(|s| s.to_string()).collect()
}
