use std::collections::HashMap;
use std::fs;

pub type Address = String;
pub type Domain = String;

const HOSTS: &str = "/etc/hosts";

pub fn parse() -> Result<HashMap<Domain,Address>,Box<dyn std::error::Error>> {
    let mut map: HashMap<Address,Domain> = HashMap::new();
    let bytes = match fs::read(HOSTS) {
        Ok(bytes) => bytes,
        Err(e) => return Err(Box::new(e)),
    };
    let file = match String::from_utf8(bytes) {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };
    let lines = file.lines().map(|s| {
        match s.find('#') {
            Some(i ) => &s[..i],
            None => s,
        }
    }).filter(|s| !s.is_empty()).collect::<Vec<&str>>();
    println!("{:#?}",lines);
    for line in lines {
        let words = line.split_whitespace().collect::<Vec<&str>>();
        let addr: Address = String::from(words[0]);
        let domains: Vec<Domain> = words[1..].iter().map(|s| String::from(*s)).collect::<Vec<Domain>>();
        for domain in domains {
            let _ = map.insert(domain, addr.clone());
        }
    }
    return Ok(map);
}

pub fn is_taken(host: Domain) -> Result<bool,Box<dyn std::error::Error>> {
    let map = match parse() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    return Ok(map.keys().any(|key| key == &host));
}