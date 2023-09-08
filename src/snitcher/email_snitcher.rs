use super::Snitcher;
use std::error::Error;

pub struct EmailSnitcher {}

impl Snitcher for EmailSnitcher {
    fn snitch(&self, message: String) -> Result<(), Box<dyn Error>> {
        println!("{message}");
        Ok(())
    }
}
