use crate::backends::{Backend, InvalidBackendError, Paste};
use serde::Deserialize;

use std::fmt::Debug;

#[derive(Debug)]
pub struct Bin<'a> {
    backend: Backend,
    config: &'a BinConfig,
}

#[derive(Debug, Deserialize)]
pub struct BinConfig {
    pub host: String,
}

impl<'a> Bin<'a> {
    pub fn from_str(backend: &'a str, cfg: &'a BinConfig) -> Result<Bin<'a>, InvalidBackendError> {
        let backend = Backend::get_backend(backend)?;

        Ok(
            Bin {
                backend: backend,
                config: cfg,
            }
        )
    }

    // pub fn apply_config(config: Config) {
    //     unimplemented!();
    // }

    pub fn post(&self, body: &str) -> Result<Paste, Box<dyn std::error::Error>> {
        self.backend.post(body, &self.config.host)
    }
}

#[test]
fn bin_from_nonexistent_backend() {
    let cfg = BinConfig{
        host: "bleh".to_string(),
    };

    assert!(Bin::from_str("", &cfg).is_err());
    assert!(Bin::from_str("non_existent_backend_123", &cfg).is_err());
}

#[test]
fn create_bin_from_str() -> Result<(), String> {
    let cfg = BinConfig {
        host: "bleh".to_string(),
    };

    let bin = Bin::from_str("clbin", &cfg)
        .expect("Cannot create Bin from `clbin`!");
    match bin.backend {
        Backend::Clbin => Ok(()),
        _ => Err(String::from("The created bin's backend wasn't clbin!"))
    }
}