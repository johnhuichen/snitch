use std::collections::HashMap;

use sysinfo::{ProcessExt, System, SystemExt};

use crate::debounced_messenger::DebouncedMessenger;

use super::Spy;

pub struct ProcessSpy {
    proc_targets: HashMap<String, String>,
    debounced_messenger: DebouncedMessenger,
}

impl ProcessSpy {
    pub fn new(proc_targets: HashMap<String, String>) -> Self {
        ProcessSpy {
            proc_targets,
            debounced_messenger: DebouncedMessenger::new(),
        }
    }
}

impl Spy for ProcessSpy {
    fn get_message(&mut self) -> Option<String> {
        let sys = System::new_all();

        for (_, process) in sys.processes() {
            if let Some(message) = self.proc_targets.get(process.name()) {
                return self
                    .debounced_messenger
                    .debounce_message(Some(message.to_string()));
            }
        }

        None
    }
}
