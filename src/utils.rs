use std::fs::File;
use std::io::{BufRead, BufReader, Read};

pub fn read_lines(filename: &str) -> Vec<String> {
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

pub fn read_file(filename: &str) -> String {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Error opening file: {}", filename),
    };

    let mut file_contents = String::new();
    if let Err(_) = file.read_to_string(&mut file_contents) {
        panic!("Error reading file: {}", filename);
    }

    return file_contents;
}
