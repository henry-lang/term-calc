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
    pub mode: Mode,
    pub debug: bool,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Self {
        match fs::read_to_string(&path) {
            Ok(data) => toml::from_str::<Self>(&data).unwrap(),
            Err(_) => Self::default(),
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
