use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut input: String = String::new();
    println!("Enter the name of the file you want to read: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let file: File = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("The file you entered does not exist");
                }
                _ => {
                    println!("Failed to open the file with an error: {}", error);
                }
            }
            return;
        }
    };

    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            }
            Err(error) => {
                println!("Failed to read line: {}", error);
            }
        }
    }
}
