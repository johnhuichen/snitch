use super::Spy;
use crate::config::Config;
use crate::snitcher::Snitcher;
use dns_lookup;
use procfs;
use std::{net::IpAddr, thread, time::Duration};

struct TCPSpy {
    tcp_targets: Vec<String>,
}

impl Spy for TCPSpy {
    fn new(config: &Config) -> Self {
        TCPSpy {
            tcp_targets: config.get_tcp_targets(),
        }
    }
    fn spy_for(&self, snitcher: &impl Snitcher) {
        let tcp_targets = self.tcp_targets.clone();
        let sender = snitcher.get_sender();
        let thread = thread::spawn(move || {
            for entry in procfs::net::tcp().unwrap().iter() {
                let ip = entry.remote_address.ip().to_string();
                let ip: IpAddr = ip.parse().unwrap();

                let host = dns_lookup::lookup_addr(&ip).unwrap_or_default();
                if tcp_targets.contains(&host) {
                    sender.lock().unwrap().send(host);
                }
            }
            thread::sleep(Duration::from_secs(5));
        });
        thread.join().unwrap();
    }
}
