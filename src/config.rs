use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode {
    #[serde(alias = "radians")]
    Radians,
    #[serde(alias = "degrees")]
    Degrees,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    mode: Mode,
    debug: bool,
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> Config {
        let config: Config =
            toml::from_str(&fs::read_to_string(&path).unwrap_or_else(|_| "".into())).unwrap();
        dbg!(config)
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
