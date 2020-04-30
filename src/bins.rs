use crate::backends::{Backend, Paste};
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
    pub fn new(backend: Backend, cfg: &'a BinConfig) -> Bin<'a> {
        Bin {
            backend: backend,
            config: cfg,
        }
    }

    pub fn post(&self, body: &str) -> Result<Paste, Box<dyn std::error::Error>> {
        self.backend.post(body, &self.config.host)
    }
}
