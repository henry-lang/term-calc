use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Mode {
    Radians,
    Degrees,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    mode: Mode,
    debug: bool,
}

impl Config {
    pub fn load_or_create(path: impl AsRef<Path>) -> Config {
        match fs::read_to_string(&path) {
            Ok(data) => toml::from_str::<Config>(&data).unwrap(),

            Err(_) => {
                let default_toml = toml::to_string(&Self::default()).unwrap();
                let _ = fs::write(&path, default_toml); // If the config doesn't save it really doesn't matter.
                Self::default()
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: Mode::Radians,
            debug: false,
        }
    }
}
