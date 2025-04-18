#![allow(dead_code)]

use serde_derive::Deserialize;
use toml::{map::Map, Value};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub path: String,
    pub uploads: String,
    pub servers: ServerConfig,
    pub cgi: CommonGatewayInterface,
}

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub index: String,
    pub not_found: String,
    pub instance: Vec<Server>, // On stocke les serveurs dans un Vec
    pub aliases: Option<Map<String,Value>>
}

#[derive(Deserialize,Debug)]
pub struct CommonGatewayInterface {
    pub bindings: Option<Map<String,Value>>,
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
