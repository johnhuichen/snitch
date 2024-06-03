use crate::debounced_messenger::DebouncedMessenger;

use super::Spy;
use dns_lookup;
use procfs::net::TcpState;
use std::{collections::HashMap, net::IpAddr};

pub struct TCPSpy {
    tcp_targets: HashMap<IpAddr, String>,
    debounced_messenger: DebouncedMessenger,
}

impl TCPSpy {
    pub fn new(tcp_targets: HashMap<String, String>) -> Self {
        let tcp_targets: HashMap<IpAddr, String> =
            tcp_targets.iter().flat_map(TCPSpy::host_to_ip).collect();
        TCPSpy {
            debounced_messenger: DebouncedMessenger::new(),
            tcp_targets,
        }
    }

    fn host_to_ip((host, message): (&String, &String)) -> Vec<(IpAddr, String)> {
        match dns_lookup::lookup_host(host) {
            Ok(ips) => ips
                .into_iter()
                .map(|ip: IpAddr| (ip, message.to_string()))
                .collect(),
            Err(_) => Vec::new(),
        }
    }

    fn do_get_message(&mut self) -> Option<String> {
        let tcp_targets = self.tcp_targets.clone();

        for entry in procfs::net::tcp().unwrap().iter() {
            let ip = entry.remote_address.ip();
            if tcp_targets.contains_key(&ip) && entry.state == TcpState::Established {
                let message = tcp_targets.get(&ip).unwrap();
                return Some(message.to_string());
            }
        }
        None
    }
}

impl Spy for TCPSpy {
    fn get_message(&mut self) -> Option<String> {
        let maybe_message = self.do_get_message();
        self.debounced_messenger.debounce_message(maybe_message)
    }
}
