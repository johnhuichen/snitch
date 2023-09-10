use std::time::{Duration, SystemTime};

use log::info;

pub struct DebouncedMessenger {
    wait_in_secs: Duration,
    start_time: Option<SystemTime>,
    stop_time: Option<SystemTime>,
}

impl DebouncedMessenger {
    pub fn new(wait_in_secs: Duration) -> Self {
        DebouncedMessenger {
            wait_in_secs,
            start_time: None,
            stop_time: None,
        }
    }

    pub fn debounce_message(&mut self, maybe_message: Option<String>) -> Option<String> {
        match maybe_message {
            Some(message) => match self.start_time {
                Some(time) if time.elapsed().expect("System time error") > self.wait_in_secs => {
                    info!("Session started");
                    self.stop_time = None;
                    return Some(message);
                }
                None => {
                    self.start_time = Some(SystemTime::now());
                }
                _ => {}
            },
            None => match self.stop_time {
                Some(time) if time.elapsed().expect("System time error") > self.wait_in_secs => {
                    info!("Session stopped");
                    self.start_time = None;
                    return None;
                }
                None => {
                    self.stop_time = Some(SystemTime::now());
                }
                _ => {}
            },
        }

        None
    }
}
