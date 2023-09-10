use confy;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

static APP_NAME: &str = "snitch";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    tcp_targets: HashMap<String, String>,
    proc_targets: HashMap<String, String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            tcp_targets: HashMap::from([(
                String::from("www.youtube.com"),
                String::from("John is being lazy and watching Youtube"),
            )]),
            proc_targets: HashMap::from([(
                String::from("some_process"),
                String::from("John is being lazy and playing some_process"),
            )]),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cfg: Config = confy::load(APP_NAME, None)?;
        log::info!("Config {:#?}", cfg);

        Ok(cfg)
    }

    pub fn get_tcp_targets(&self) -> HashMap<String, String> {
        self.tcp_targets.clone()
    }
}
