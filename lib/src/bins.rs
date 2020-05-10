use crate::backends::{Backend, Paste};
use serde::Deserialize;

use std::fmt::Debug;

#[derive(Debug, Deserialize)]
pub struct Bin {
    backend: Backend,
    host: String,
}

impl Bin {
    pub fn post(&self, body: &str) -> Result<Paste, Box<dyn std::error::Error>> {
        self.backend().post(body, self.host())
    }

    fn host(&self) -> &str {
        &self.host
    }

    fn backend(&self) -> &Backend {
        &self.backend
    }
}
