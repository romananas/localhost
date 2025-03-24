#![allow(dead_code)]

use super::args::Args;
use super::config::Config;
use super::ip::IPv4;

use std::collections::HashMap;

pub struct Opts {
    path: String,

    index: String,
    links: Option<HashMap<String, String>>,

    instances: HashMap<String,Vec<u32>>
}

impl Opts {
    pub fn from_args(a: Args) -> Self {
        let mut instances: HashMap<String,Vec<u32>> = HashMap::new();
        for s in a.addr {
            let ip = IPv4::from(s.as_str()).unwrap();
            match instances.get_mut(&ip.addr()) {
                Some(v) => v.push(ip.port),
                None => { instances.insert(ip.addr(), vec![ip.port]); }
            }
        }
        let mut links: HashMap<String, String> = HashMap::new();
        let links = match a.aliases {
            Some(al) => {
                for alias in al {
                    let tmp = alias.split(":").collect::<Vec<&str>>();
                    links.insert(tmp[0].to_string(), tmp[1].to_string());
                }
                Some(links)
            }
            None => None,
        };
        Self { path: a.path, index: a.entry_point, links: links, instances: instances }
    }

    pub fn from_config(c: Config) -> Self {
        let mut instances: HashMap<String,Vec<u32>> = HashMap::new();
        for s in c.servers.instance {
            instances.insert(s.address, s.ports);
        }
        Self { path: c.path, index: c.servers.entry_point, links: c.servers.aliases, instances: instances }
    }
}