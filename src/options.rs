use toml::Value;

use super::config::Config;

use std::path::Path;
use std::collections::HashMap;

fn map_to_hashmap(map: toml::map::Map<String,Value>) -> HashMap<String,String>{
    let mut hash = HashMap::new();
    for (a,b) in map {
        let b = match b.as_str() {
            Some(v) => v.to_string(),
            None => panic!("aliases must be a string therefore {} is not",b),
        };
        hash.insert(a, b);
    };
    hash
}

#[derive(Debug,Clone)]
pub struct Opts {
    pub path: String,
    pub upload: String,
    pub index: String,
    pub not_found: String,
    pub links: HashMap<String, String>,
    pub instances: HashMap<String,Vec<u32>>,
    pub cgi_binds: HashMap<String,String>,
    pub hosts: HashMap<String,String>,
}

impl Opts {
    pub fn from_config(c: Config) -> Result<Self,String> {
        let mut instances: HashMap<String,Vec<u32>> = HashMap::new();
        for s in &c.servers.instance {
            instances.insert(s.address.clone(), s.ports.clone());
        }
        // println!("x = {}",c.servers.aliases);
        let links = match c.servers.aliases.clone() {
            Some(v) => map_to_hashmap(v),
            None => HashMap::new(),
        };
        let cgi_binds: HashMap<String,String> = match c.cgi.bindings {
            Some(v) => map_to_hashmap(v),
            None => {
                let mut default = HashMap::new();
                default.insert(String::from("py"), String::from("python3"));
                default
            },
        };
        let path = match verify_dir_format(&c.path) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
            // changing working directory
        match std::env::set_current_dir(path.clone()) {
            Ok(_) => (), 
            Err(e) => {
                return Err(format!("{}",e));
            },
        }
        let upload =  match verify_dir_format(&c.uploads) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let mut hosts: HashMap<String,String> = HashMap::new();
        for instance in &c.servers.instance {
            if let Some(hosts_list) = &instance.hosts {
                for domain in hosts_list {
                    if hosts.contains_key(domain) {
                        return Err(format!("the hostname '{}' has been entered twice", domain));
                    } else {
                        hosts.insert(domain.to_string(), instance.address.clone());
                    }
                }
            }
        }
        Ok(Self { path, index: c.servers.index, links: links, not_found: c.servers.not_found, instances: instances, cgi_binds: cgi_binds, upload, hosts})
    }

    /// Generate every addresses/port combinations for every instances
    pub fn address_combinations(&self) -> Vec<String> {
        let instances = self.instances.clone();
        instances.iter().flat_map(|(addr,ports)| {
            ports.iter().map(|p| {format!("{}:{}",addr,p)}).collect::<Vec<String>>()
        }).collect::<Vec<String>>()
    }
}

fn verify_dir_format(dir: &str) -> Result<String, String>{
    if !Path::new(dir).exists() {
        return Err(format!("{} is does not exist",dir));
    }
    if !Path::new(dir).is_dir() {
        return Err(format!("{} is not a directory",dir));
    }
    if !dir.ends_with("/") {
        return Ok(format!("{}/",dir));
    } else {
        return Ok(String::from(dir));
    }
}