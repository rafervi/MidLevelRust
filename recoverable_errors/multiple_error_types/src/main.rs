use std::{fs, num::ParseIntError, error};

fn main() {
    let i = parse_file("example.txt");
    match i {
        Ok(i) => println!("{i}"),
        Err(e) => {
            match e {
                ParseFileError::File=> {
                    // ...
                },
                ParseFileError::Parse(e) => {
                    // ...
                }
            }
        }
    }
}

enum ParseFileError {
    File,
    Parse(ParseIntError),
}

fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
    let s = fs::read_to_string(filename)
                        .map_err(|e| ParseFileError::File)?;
    let i = s.parse()
                        .map_err(|e| ParseFileError::Parse(e))?;
    //read_to_string and parse will show different types of errors
    Ok(i)
}
