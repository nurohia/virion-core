use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub enum Mode {
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "relay")]
    Relay,
    #[serde(rename = "server")]
    Server,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub mode: Mode,
    pub listen: String,
    pub password: String,
    pub next: Option<String>,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
}
