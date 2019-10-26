use serde::Deserialize;
use config::{self, ConfigError, File, FileFormat};
use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use crate::bins::BinConfig;
use std::str;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

lazy_static! {
    static ref CFG: config::Config = {
        let mut c = config::Config::new();
        c.merge(File::from_str(
            str::from_utf8(
                Assets::get("default_cfg.toml")
                    .unwrap()
                    .as_ref()
            ).unwrap(),
            FileFormat::Toml,
        ));
        c
    };
}

#[derive(Debug, Deserialize)]
pub struct PstConfig {
    bin: String,
}

impl PstConfig {
    pub fn bin(&self) -> &str {
        &self.bin
    }
}

pub fn pst_config() -> Result<PstConfig, ConfigError> {
    CFG.clone().try_into()
}

pub fn bin_config(name: &str) -> Result<BinConfig, ConfigError> {
    CFG.get(name)
}
