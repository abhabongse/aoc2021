use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn reader_from_file(input_file: Option<&str>) -> Box<dyn BufRead> {
    match input_file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(fs::File::open(filename).unwrap())),
    }
}
