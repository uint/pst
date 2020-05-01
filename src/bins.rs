use crate::backends::{Backend, Paste};
use serde::Deserialize;

use std::fmt::Debug;

pub trait Bin {
    fn host(&self) -> &str;
    fn backend(&self) -> &Backend;

    fn post(&self, body: &str) -> Result<Paste, Box<dyn std::error::Error>> {
        self.backend().post(body, self.host())
    }
}

#[derive(Debug)]
pub struct BinOwned<'a> {
    backend: Backend,
    config: &'a BinConfig,
}

impl<'a> BinOwned<'a> {
    pub fn new(backend: Backend, cfg: &'a BinConfig) -> BinOwned<'a> {
        BinOwned {
            backend: backend,
            config: cfg,
        }
    }
}

impl<'a> Bin for BinOwned<'a> {
    fn host(&self) -> &str {
        &self.config.host
    }

    fn backend(&self) -> &Backend {
        &self.backend
    }
}

#[derive(Debug, Deserialize)]
pub struct BinConfig {
    pub host: String,
}
