use dns_lookup;
use procfs::net::tcp;
use std::{collections, net};

fn main() {
    let ignore_ips = ["0.0.0.0", "127.0.0.1"];

    let hosts = tcp()
        .unwrap()
        .into_iter()
        .filter_map(|entry| {
            let ip = entry.remote_address.ip().to_string();
            if ignore_ips.contains(&ip.as_str()) {
                return None;
            }
            let ip: net::IpAddr = ip.parse().unwrap();
            dns_lookup::lookup_addr(&ip).ok()
        })
        .collect::<collections::HashSet<_>>();

    println!("{:?}", hosts);
}
