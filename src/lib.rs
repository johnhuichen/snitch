use confy;
use dns_lookup;
use procfs;
use std::{default::Default, error::Error, net, thread, time::Duration};

#[derive(Debug)]
struct Config {
    snitch_target_hosts: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            snitch_target_hosts: vec![],
        }
    }
}

fn tcp_snitchable() -> bool {
    let ignore_ips = ["0.0.0.0", "127.0.0.1"];
    let target_hosts = ["crawl.kelbi.org"];

    for entry in procfs::net::tcp().unwrap().iter() {
        let ip = entry.remote_address.ip().to_string();
        if ignore_ips.contains(&ip.as_str()) {
            continue;
        }
        let ip: net::IpAddr = ip.parse().unwrap();

        let host = dns_lookup::lookup_addr(&ip).unwrap_or_default();
        if target_hosts.contains(&host.as_ref()) {
            return true;
        }
    }

    return false;
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let cfg = confy::load("my-app-name", None)?;
    dbg!(cfg);

    let thread = thread::spawn(|| {
        let snitchable = tcp_snitchable();
        dbg!(snitchable);
        thread::sleep(Duration::from_secs(5));
    });

    thread.join().unwrap_or_default();

    Ok(())
}
