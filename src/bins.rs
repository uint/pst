use crate::backends::{Backend, Paste};
use serde::Deserialize;

use std::fmt::Debug;

pub trait Bin: Debug {
    fn host(&self) -> &str;
    fn backend(&self) -> &Backend;

    fn post(&self, body: &str) -> Result<Paste, Box<dyn std::error::Error>> {
        self.backend().post(body, self.host())
    }
}

#[derive(Debug, Deserialize)]
pub struct BinOwned {
    backend: Backend,
    host: String,
}

impl Bin for BinOwned {
    fn host(&self) -> &str {
        &self.host
    }

    fn backend(&self) -> &Backend {
        &self.backend
    }
}

#[derive(Debug)]
pub struct BinShared<'a> {
    backend: &'a Backend,
    host: &'a str,
}

impl BinShared<'_> {
    pub fn new<'a>(backend: &'a Backend, host: &'a str) -> BinShared<'a> {
        BinShared {
            backend,
            host,
        }
    }
}

impl Bin for BinShared<'_> {
    fn host(&self) -> &str {
        self.host
    }

    fn backend(&self) -> &Backend {
        self.backend
    }
}
