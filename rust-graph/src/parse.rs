use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Function to parse each line in dataset
pub fn parse_file<P>(filename: P) -> io::Result<Vec<(u64, u64)>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // Skip comments and empty lines
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(from), Ok(to)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                edges.push((from, to));
            }
        }
    }
    Ok(edges)
}
