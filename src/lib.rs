use config::Config;
use job_delegator::{JobDelegator, Snitchers, Spies};
use snitcher::email_snitcher::EmailSnitcher;
use spy::tcp_spy::TCPSpy;
use std::error::Error;

mod config;
mod job_delegator;
mod logger;
mod snitcher;
mod spy;

pub fn run() -> Result<(), Box<dyn Error>> {
    logger::init()?;

    let cfg = Config::new()?;
    let spies: Spies = vec![Box::new(TCPSpy::new(cfg.get_tcp_targets()))];
    let snitchers: Snitchers = vec![Box::new(EmailSnitcher {})];

    JobDelegator::run(spies, snitchers)
}
