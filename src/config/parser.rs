#![allow(dead_code)]

use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path: String,
    pub servers: ServerConfig,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub entry_point: String,
    pub aliases: Option<HashMap<String, String>>,
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
