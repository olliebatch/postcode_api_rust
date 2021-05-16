use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(default = "defaults::port")]
    pub port: u16,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        let config = envy::from_env::<Self>()?;
        Ok(config)
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "- Port: {:}", self.port)
    }
}

mod defaults {
    pub const fn port() -> u16 {
        8080
    }
}
