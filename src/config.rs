use std::str;
use std::path::PathBuf;
use std::error::Error;
use std::io::Write;
use std::collections::HashMap;
use std::fmt;

use serde::Deserialize;
use config::{self, FileFormat};
use crate::bins::BinOwned;
use rust_embed::RustEmbed;
use app_dirs::{self, AppDataType, AppInfo, AppDirsError};

const APP_INFO: AppInfo = AppInfo{name: "pst", author: "Tomasz Kurcz"};

type BoxError = Box<dyn Error>;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

#[derive(Debug, Deserialize)]
pub struct PstConfig {
    default_bin: String,
    bins: HashMap<String, config::Value>,
}

impl PstConfig {
    pub fn new() -> Result<PstConfig, BoxError> {
        let mut c = config::Config::new();

        // The default config, embedded using `rust_embed`
        c.merge(config::File::from_str(
            str::from_utf8(
                Assets::get("default_cfg.toml")
                    .unwrap()
                    .as_ref()
            )?,
            FileFormat::Toml,
        ))?;

        // User config found in the default location (usually home dir)
        if let Ok(path) = path_to_user_cfg() {
            let path = path.to_str().unwrap();
            
            #[cfg(debug)]
            eprintln!("User config path: {:?}", path);

            c.merge(config::File::with_name(path).required(false))?;
        }

        Ok(c.try_into()?)
    }

    pub fn default_bin_name(&self) -> &str {
        &self.default_bin
    }

    pub fn bin<'s>(&self, name: &'s str) -> Result<BinOwned, BoxError> {
        let value = self.bins.get(name).ok_or(InvalidBinError::new(name))?;
        Ok(value.clone().try_into()?)
    }
}

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
