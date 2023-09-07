use confy;
use serde::{Deserialize, Serialize};
use std::error::Error;

static APP_NAME: &str = "snitch";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    tcp_targets: Vec<String>,
    proc_targets: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cfg: Config = confy::load(APP_NAME, None)?;
        log::info!("Config {:?}", cfg);

        Ok(cfg)
    }

    pub fn get_tcp_targets(&self) -> Vec<String> {
        self.tcp_targets
    }
}
