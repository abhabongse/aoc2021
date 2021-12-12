//! Implements a simplified version of program argument parser.
use std::io;
use std::path::PathBuf;

/// Type which represents the main input source of the program.
/// It can be either a standard input or path to a text file.
pub enum InputSrc {
    Stdin,
    File(PathBuf),
}

impl InputSrc {
    /// Transforms a program argument into the [`InputSrc`] representation.
    /// The absence of the argument or the path string `'-'` indicates the standard input as source;
    /// otherwise, the string shall be used as a path to the text file.
    pub fn from_arg(arg: Option<&str>) -> Self {
        match arg {
            None | Some("-") => InputSrc::Stdin,
            Some(s) => InputSrc::File(PathBuf::from(s)),
        }
    }

    /// Obtains a boxed buffered reader based on the input source.
    pub fn get_reader(&self) -> anyhow::Result<Box<dyn io::BufRead>> {
        Ok(match self {
            Self::Stdin => Box::new(io::BufReader::new(io::stdin())),
            Self::File(name) => Box::new(io::BufReader::new(std::fs::File::open(name)?)),
        })
    }
}
