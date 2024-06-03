use crate::snitcher::Snitcher;
use crate::spy::Spy;
use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// TODO: move to config file under project root
const SLEEP_INTERVAL: Duration = Duration::from_secs(60);

pub type Spies = Vec<Box<dyn Spy + Send>>;
pub type Snitchers = Vec<Box<dyn Snitcher + Send>>;

pub struct JobDelegator {}

impl JobDelegator {
    pub fn run(mut spies: Spies, snitchers: Snitchers) -> Result<(), Box<dyn Error>> {
        let (sender, receiver) = mpsc::channel::<String>();

        let handler = thread::spawn(move || loop {
            let message = receiver.recv().expect("Panic when receiving message");
            for snitcher in snitchers.iter() {
                snitcher.snitch(message.clone()).unwrap();
            }
        });

        thread::spawn(move || loop {
            for spy in spies.iter_mut() {
                if let Some(message) = spy.get_message() {
                    sender.send(message).expect("Panic when sending {message}");
                }
            }

            thread::sleep(SLEEP_INTERVAL);
        });

        handler.join().expect("Panic in receiver thread join");

        Ok(())
    }
}
