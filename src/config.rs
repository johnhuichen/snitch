use confy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

static APP_NAME: &str = "snitch";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    tcp_targets: HashMap<String, String>,
    proc_targets: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cfg: Config = confy::load(APP_NAME, None)?;
        log::info!("Config {:?}", cfg);

        Ok(cfg)
    }

    pub fn get_tcp_targets(&self) -> HashMap<String, String> {
        self.tcp_targets.clone()
    }
}
