use std::time::{Duration, SystemTime};

const WAIT_IN_SECS: Duration = Duration::from_secs(400);

pub struct DebouncedMessenger {
    start_time: Option<SystemTime>,
    marker_time: Option<SystemTime>,
}

impl DebouncedMessenger {
    pub fn new() -> Self {
        DebouncedMessenger {
            start_time: None,
            marker_time: None,
        }
    }

    pub fn debounce_message(&mut self, maybe_message: Option<String>) -> Option<String> {
        if maybe_message.is_some() && self.start_time.is_none() {
            if self.marker_time.is_none() {
                log::info!("Mark potential session start");
                self.marker_time = Some(SystemTime::now());
                return None;
            }

            if self
                .marker_time
                .unwrap()
                .elapsed()
                .expect("System time error")
                >= WAIT_IN_SECS
            {
                log::info!("Confirm session start");
                self.start_time = self.marker_time.to_owned();
                self.marker_time = None;
                return maybe_message;
            }

            log::debug!("Waiting to confirm session start");
            return None;
        }

        if maybe_message.is_some() && self.start_time.is_some() {
            log::debug!("In session");
            return None;
        }

        if maybe_message.is_none() && self.start_time.is_none() {
            log::debug!("Nothing happened");
            return None;
        }

        if maybe_message.is_none() && self.start_time.is_some() {
            if self.marker_time.is_none() {
                log::info!("Mark potential session stop");
                self.marker_time = Some(SystemTime::now());
                return None;
            }

            if self
                .marker_time
                .unwrap()
                .elapsed()
                .expect("System time error")
                >= WAIT_IN_SECS
            {
                log::info!(
                    "Confirm session stop, duration: {:?}",
                    self.start_time
                        .unwrap()
                        .elapsed()
                        .expect("System time error")
                );
                self.start_time = None;
                self.marker_time = None;
                return None;
            }

            log::debug!("Waiting to confirm session stop");
            return None;
        }

        None
    }
}
