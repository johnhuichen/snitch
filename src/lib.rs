use crate::{
    config::Config,
    job_delegator::{JobDelegator, Snitchers, Spies},
    snitcher::email_snitcher::EmailSnitcher,
    spy::{process_spy::ProcessSpy, tcp_spy::TCPSpy},
};
use std::error::Error;

mod config;
mod debounced_messenger;
mod job_delegator;
mod logger;
mod snitcher;
mod spy;

pub fn run() -> Result<(), Box<dyn Error>> {
    logger::init()?;

    let cfg = Config::new()?;
    let spies: Spies = vec![
        Box::new(TCPSpy::new(cfg.get_tcp_targets())),
        Box::new(ProcessSpy::new(cfg.get_proc_targets())),
    ];
    let snitchers: Snitchers = vec![Box::new(EmailSnitcher::new(cfg.get_smtp_info()))];

    JobDelegator::run(spies, snitchers)
}
