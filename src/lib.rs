use std::error::Error;

mod config;
mod tcp_snitch;

pub fn run() -> Result<(), Box<dyn Error>> {
    let appConfig = config::Config::new();
    tcp_snitch::run();
    Ok(())
}
