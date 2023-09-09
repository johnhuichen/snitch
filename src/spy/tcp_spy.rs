use super::Spy;
use dns_lookup;
use std::collections::HashMap;
use std::net::IpAddr;

pub struct TCPSpy {
    host_map: HashMap<String, String>,
    tcp_targets: HashMap<String, String>,
}

impl TCPSpy {
    pub fn new(tcp_targets: HashMap<String, String>) -> Self {
        TCPSpy {
            host_map: HashMap::new(),
            tcp_targets,
        }
    }

    fn get_host(&mut self, ip: String) -> &str {
        self.host_map.entry(ip.to_string()).or_insert_with(|| {
            let ip_addr: IpAddr = ip.parse().expect("Panic when parsing ip {ip}");
            dns_lookup::lookup_addr(&ip_addr).unwrap_or_default()
        })
    }
}

impl Spy for TCPSpy {
    fn get_message(&mut self) -> Option<String> {
        let tcp_targets = self.tcp_targets.clone();
        for entry in procfs::net::tcp()
            .expect("Panic when getting tcp connections")
            .iter()
        {
            let ip = entry.remote_address.ip().to_string();
            let host = self.get_host(ip);
            if let Some(message) = tcp_targets.get(host) {
                log::info!("found a tcp target {host}");
                return Some(message.to_string());
            }
        }
        None
    }
}
