use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();
    log::info!("Logger started");

    Ok(())
}
