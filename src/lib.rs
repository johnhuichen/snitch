use std::error::Error;

mod config;
mod logger;
mod snitcher;

pub fn run() -> Result<(), Box<dyn Error>> {
    logger::init()?;
    log::info!("Starting Snitch App");

    let cfg = config::Config::new()?;

    // snitcher::run();

    Ok(())
}
