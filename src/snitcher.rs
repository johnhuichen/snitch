use std::error::Error;

pub mod email_snitcher;

pub trait Snitcher {
    fn snitch(&self, message: String) -> Result<(), Box<dyn Error>>;
}
