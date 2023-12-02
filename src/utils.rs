use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect(&format!("[File {} not found!]", filename));
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(text) => lines.push(text),
            Err(err) => eprintln!("Error reading line: {}", err),
        }
    }

    return lines;
}
