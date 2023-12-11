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

pub fn read_chars(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).expect("Failed to open file");

    let reader = BufReader::new(file);
    let mut vec: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        if let Ok(line_contents) = line {
            let chars: Vec<char> = line_contents.chars().filter(|&c| c != '\n').collect();
            vec.push(chars);
        }
    }

    return vec;
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

pub fn print_mat<T>(mat: &[Vec<T>])
where
    T: std::fmt::Display,
{
    for line in mat {
        for c in line {
            print!("{} ", c);
        }
        println!();
    }
}
