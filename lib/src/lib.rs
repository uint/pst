pub mod bins;
pub mod backends;

pub use bins::Bin;

use std::str;
use std::error::Error;
use std::collections::{HashMap, hash_map::Keys};
use std::fmt;

use serde::Deserialize;

type BoxError = Box<dyn Error>;

#[derive(Debug, Deserialize)]
pub struct Pst {
    default_bin: String,
    bins: HashMap<String, Bin>,
}

impl Pst {
    pub fn new() -> Self {
        Pst {
            default_bin: "termbin".to_string(),
            bins: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Pst {
            default_bin: String::new(),
            bins: HashMap::new(),
        }
    }

    pub fn default_bin_name(&self) -> &str {
        &self.default_bin
    }

    pub fn bin<'s>(&self, name: &'s str) -> Result<&Bin, BoxError> {
        Ok(self.bins.get(name).ok_or(InvalidBinError::new(name))?)
    }

    pub fn bin_names(&self) -> BinNames<'_> {
        self.bins.keys()
    }
}

type BinNames<'a> = Keys<'a, String, Bin>;

#[derive(Debug)]
pub struct InvalidBinError {
    bin_name: String,
}

impl InvalidBinError {
    fn new(bin_name: &str) -> InvalidBinError {
        InvalidBinError {
            bin_name: bin_name.to_string(),
        }
    }
}

impl fmt::Display for InvalidBinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Undefined bin `{}`", self.bin_name)
    }
}

impl Error for InvalidBinError {}
