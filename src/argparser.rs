use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

/// Represents an input source, which is either a standard input or a text file.
pub enum InputSrc {
    Stdin,
    File(PathBuf),
}

impl InputSrc {
    /// Transforms a program argument into the input source representation.
    /// Absence of argument or '-' indicates the standard input as source;
    /// otherwise, the string will be used as path to the file.
    pub fn from_arg(arg: Option<&str>) -> Self {
        match arg {
            None | Some("-") => InputSrc::Stdin,
            Some(s) => InputSrc::File(PathBuf::from(s)),
        }
    }

    /// Obtains a buffer reader from the input source.
    pub fn to_reader(&self) -> anyhow::Result<Box<dyn BufRead>> {
        Ok(match self {
            Self::Stdin => Box::new(BufReader::new(io::stdin())),
            Self::File(name) => Box::new(BufReader::new(std::fs::File::open(name)?)),
        })
    }
}
