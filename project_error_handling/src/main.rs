use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("non_existing_file.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                _=> {
                    panic!("Error opening file: {}", error)
                }
            }
        },
    };
}
