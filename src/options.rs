#![allow(dead_code)]

use super::args::Args;
use super::config::Config;
use super::ip::IPv4;

use std::collections::HashMap;

#[derive(Debug,Clone)]
pub struct Opts {
    pub path: String,
    pub index: String,
    pub not_found: String,
    pub links: HashMap<String, String>,
    pub instances: HashMap<String,Vec<u32>>
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
        let links: HashMap<String, String> = HashMap::new();
        // let links = match a.links {
        //     Some(al) => {
        //         for alias in al {
        //             let tmp = alias.split(":").collect::<Vec<&str>>();
        //             links.insert(tmp[0].to_string(), tmp[1].to_string());
        //         }
        //         links
        //     }
        //     None => HashMap::new(),
        // };
        Self { path: a.path, index: a.entry_point, not_found: a.not_found ,links: links, instances: instances }
    }

    pub fn from_config(c: Config) -> Self {
        let mut instances: HashMap<String,Vec<u32>> = HashMap::new();
        for s in c.servers.instance {
            instances.insert(s.address, s.ports);
        }
        // println!("x = {}",c.servers.aliases);
        let aliases = match c.servers.aliases.clone() {
            Some(v) => v,
            None => toml::map::Map::new(),
        };
        let mut links: HashMap<String,String> = HashMap::new();
        for (file,path) in aliases {
            let path = match path.as_str() {
                Some(v) => v.to_string(),
                None => panic!("aliases must be a string therefore {} is not",path),
            };
            links.insert(path,file );
        }
        Self { path: c.path, index: c.servers.index, links: links, not_found: c.servers.not_found, instances: instances }
    }

    /// Generate every addresses/port combinations for every instances
    /// 
    /// Result = addr:port
    pub fn address_combinations(&self) -> Vec<String> {
        let instances = self.instances.clone();
        instances.iter().flat_map(|(addr,ports)| {
            ports.iter().map(|p| {format!("{}:{}",addr,p)}).collect::<Vec<String>>()
        }).collect::<Vec<String>>()
    }
}