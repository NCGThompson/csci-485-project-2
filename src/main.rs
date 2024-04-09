use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments are provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[1]);
        std::process::exit(1);
    }

    // Extract file path from command-line arguments
    let file_path = &args[1];

    println!("File path: {}", file_path);

    // Read lines from the file
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            println!("{}", line);
        }
    } else {
        eprintln!("Error reading file: {}", file_path);
        std::process::exit(1);
    }
}

/// Read lines from a file and return them as a vector of strings.
fn read_lines<P>(file_path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}
