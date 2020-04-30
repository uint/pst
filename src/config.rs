use std::str;
use std::path::PathBuf;
use std::error::Error;
use std::io::Write;

use serde::Deserialize;
use config::{self, ConfigError, FileFormat};
use rust_embed::RustEmbed;
use app_dirs::{self, AppDataType, AppInfo, AppDirsError};

use crate::bins::BinConfig;

const APP_INFO: AppInfo = AppInfo{name: "pst", author: "Tomasz Kurcz"};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

pub struct ConfigStore {
    data: config::Config,
}

impl ConfigStore {
    pub fn new() -> Result<ConfigStore, Box<dyn Error>> {
        let mut c = config::Config::new();

        // The default config, embedded using `rust_embed`
        c.merge(config::File::from_str(
            str::from_utf8(
                Assets::get("default_cfg.toml")
                    .unwrap()
                    .as_ref()
            ).unwrap(),
            FileFormat::Toml,
        )).unwrap();

        // User config found in the default location (usually home dir)
        if let Ok(path) = path_to_user_cfg() {
            let path = path.to_str().unwrap();
            
            #[cfg(debug)]
            eprintln!("User config path: {:?}", path);

            c.merge(config::File::with_name(path).required(false))
                .unwrap();
        }

        Ok(
            ConfigStore{
                data: c,
            }
        )
    }

    pub fn pst_config(&self) -> Result<PstConfig, ConfigError> {
        self.data.clone().try_into()
    }

    pub fn bin_config(&self, name: &str) -> Result<BinConfig, ConfigError> {
        self.data.get(name)
    }
}

fn path_to_user_cfg() -> Result<PathBuf, AppDirsError> {
    Ok(app_dirs::app_root(
        AppDataType::UserConfig,
        &APP_INFO,
    )?.join("cfg"))
}

pub fn write_default_cfg() -> Result<(), Box<dyn Error>> {
    let mut path = path_to_user_cfg()?;
    path.set_extension("toml");
    let mut file = std::fs::File::create(&path)?;
    file.write_all(
        Assets::get("default_cfg.toml").unwrap().as_ref()
    )?;
    eprintln!("Default config successfully written to: {:?}", &path);
    Ok(())
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
