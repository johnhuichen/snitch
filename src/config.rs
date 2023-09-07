use confy;
use serde::{Deserialize, Serialize};
use std::error::Error;

static APP_NAME: &str = "snitch";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    snitch_target_hosts: Vec<String>,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cfg: Config = confy::load(APP_NAME, None)?;
        dbg!(&cfg);
        let test = confy::get_configuration_file_path(APP_NAME, None)?;
        dbg!(test);

        Ok(cfg)
    }
}
