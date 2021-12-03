use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Obtains a buffer reader from the given input file.
/// However, if '-' is specified, then standard input will be used as source.
pub fn reader_from_file<P: AsRef<Path>>(input_file: P) -> anyhow::Result<Box<dyn BufRead>> {
    Ok(if input_file.as_ref() == Path::new("-") {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(std::fs::File::open(input_file)?))
    })
}
