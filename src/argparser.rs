use std::io::{self, BufRead, BufReader};

pub fn reader_from_file(input_file: Option<&str>) -> anyhow::Result<Box<dyn BufRead>> {
    Ok(match input_file {
        None => Box::new(BufReader::new(io::stdin())),
        Some(filename) => Box::new(BufReader::new(std::fs::File::open(filename)?)),
    })
}
