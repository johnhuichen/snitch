use crate::debounced_messenger::DebouncedMessenger;

use super::Spy;
use dns_lookup;
use procfs::net::TcpState;
use std::{collections::HashMap, net::IpAddr};

pub struct TCPSpy {
    host_map: HashMap<String, String>,
    tcp_targets: HashMap<String, String>,
    debounced_messenger: DebouncedMessenger,
}

impl TCPSpy {
    pub fn new(tcp_targets: HashMap<String, String>) -> Self {
        TCPSpy {
            host_map: HashMap::new(),
            debounced_messenger: DebouncedMessenger::new(),
            tcp_targets,
        }
    }

    fn get_host(&mut self, ip: String) -> &str {
        self.host_map.entry(ip.to_string()).or_insert_with(|| {
            let ip_addr: IpAddr = ip.parse().expect("Panic when parsing ip {ip}");
            dns_lookup::lookup_addr(&ip_addr).unwrap_or_default()
        })
    }

    fn do_get_message(&mut self) -> Option<String> {
        let tcp_targets = self.tcp_targets.clone();

        for entry in procfs::net::tcp().unwrap().iter() {
            let ip = entry.remote_address.ip().to_string();
            let host = self.get_host(ip);
            if tcp_targets.contains_key(host) && entry.state == TcpState::Established {
                let message = tcp_targets.get(host).unwrap();
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
