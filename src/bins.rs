use crate::backends::{Backend, InvalidBackendError, Paste};

use std::fmt::Debug;

#[derive(Debug)]
pub struct Bin<'a> {
    backend: &'a Backend,
}

impl Bin<'_> {
    pub fn from_str(backend: &str) -> Result<Bin, InvalidBackendError> {
        Ok(
            Bin {
                backend: Backend::get_backend(backend)?,
            }
        )
    }

    // pub fn apply_config(config: Config) {
    //     unimplemented!();
    // }

    pub fn post(&self, body: &str) -> Result<Paste, Box<dyn std::error::Error>> {
        self.backend.post(body)
    }
}

#[test]
fn bin_from_nonexistent_backend() {
    assert!(Bin::from_str("").is_err());
    assert!(Bin::from_str("non_existent_backend_123").is_err());
}

#[test]
fn create_bin_from_str() -> Result<(), String> {
    let bin = Bin::from_str("clbin").expect("Cannot create Bin from `clbin`!");
    match bin.backend {
        Backend::Clbin => Ok(()),
        _ => Err(String::from("The created bin's backend wasn't clbin!"))
    }
}