#![allow(dead_code)]

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path: String,
    pub servers: ServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub index: String,
    pub not_found: String,
    pub instance: Vec<Server>, // On stocke les serveurs dans un Vec
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub address: String,
    pub ports: Vec<u32>,
}

pub fn get(s: &str) -> Result<Config,String>{
    let config: Config = match toml::from_str(s) {
        Ok(v) => v,
        Err(e) => return Err(format!("{}",e)),
    };
    Ok(config)
}
