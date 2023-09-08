use config::Config;
use snitcher::email_snitcher::EmailSnitcher;
use snitcher::Snitcher;
use spy::tcp_spy::TCPSpy;
use spy::Spy;
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod config;
mod logger;
mod snitcher;
mod spy;

pub fn run() -> Result<(), Box<dyn Error>> {
    logger::init()?;
    log::info!("Starting Snitch App");

    let cfg = Config::new()?;
    let email_snitcher = EmailSnitcher {};
    let mut tcp_spy = TCPSpy::new(cfg.get_tcp_targets());

    let (sender, receiver) = mpsc::channel::<Option<String>>();

    let handler = thread::spawn(move || loop {
        if let Some(message) = receiver.recv().expect("receiver thread panicked") {
            email_snitcher.snitch(message).unwrap();
        }
    });

    thread::spawn(move || loop {
        let message = tcp_spy.get_message();
        sender.send(message).expect("sender thread panicked");
        thread::sleep(Duration::from_secs(5));
    });

    handler.join().unwrap();

    Ok(())
}
