use crate::debounced_messenger::DebouncedMessenger;

use super::Spy;
use dns_lookup;
use procfs::net::TcpState;
use retry::delay::Exponential;
use retry::retry;
use std::{collections::HashMap, net::IpAddr};

pub struct TCPSpy {
    host_map: HashMap<String, String>,
    tcp_targets: HashMap<String, String>,
    debounced_messenger: DebouncedMessenger,
}

impl TCPSpy {
    pub fn new(host_targets: HashMap<String, String>) -> Self {
        // wait till network is up
        match retry(Exponential::from_millis(5000), || {
            log::info!("Waiting for network...");
            dns_lookup::lookup_host("google.com")
        }) {
            Ok(_) => log::info!("Network is up"),
            Err(_) => log::error!("Network is down"),
        }
        let ip_targets: HashMap<String, String> =
            host_targets.iter().flat_map(TCPSpy::host_to_ip).collect();
        let tcp_targets = host_targets.into_iter().chain(ip_targets).collect();

        TCPSpy {
            host_map: HashMap::new(),
            debounced_messenger: DebouncedMessenger::new(),
            tcp_targets,
        }
    }

    fn host_to_ip((host, message): (&String, &String)) -> Vec<(String, String)> {
        match retry(Exponential::from_millis(1000).take(3), || {
            dns_lookup::lookup_host(host)
        }) {
            Ok(ips) => ips
                .into_iter()
                .map(|ip: IpAddr| (ip.to_string(), message.to_string()))
                .collect(),
            Err(err) => {
                log::error!("Failed to look up host {:?}", err);
                Vec::new()
            }
        }
    }

    fn get_host(&mut self, ip: &str) -> &str {
        self.host_map.entry(ip.to_string()).or_insert_with(|| {
            let ip_addr: IpAddr = ip.parse().expect("Panic when parsing ip {ip}");
            dns_lookup::lookup_addr(&ip_addr).unwrap_or_default()
        })
    }

    fn do_get_message(&mut self) -> Option<String> {
        let tcp_targets = self.tcp_targets.clone();

        for entry in procfs::net::tcp().unwrap().iter() {
            let ip = entry.remote_address.ip().to_string();

            if tcp_targets.contains_key(&ip) && entry.state == TcpState::Established {
                let message = tcp_targets.get(&ip).unwrap();
                return Some(message.to_string());
            } else if tcp_targets.contains_key(self.get_host(&ip))
                && entry.state == TcpState::Established
            {
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
