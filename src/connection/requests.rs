use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use super::content::Content;

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub enum RequestType {
    POST,
    GET,
    DELETE,
    UNKNOWN,
}

#[derive(Debug,Clone)]
#[allow(dead_code)]
pub struct Request {
    pub rtype: RequestType,
    pub path: String,
    pub version: String,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    pub content: Content,
}


use std::io::Read; // n'oublie pas ça en haut

impl Request {
    pub fn parse(reader: &mut BufReader<TcpStream>) -> Option<Self> {
        let raw = parse(reader);

        if raw.is_empty() {
            return None;
        }

        let start_line = &raw[0];
        let mut host = String::new();
        let mut user_agent = String::new();
        let mut accept = String::new();
        let mut content_type = String::new();
        let mut content_length = 0;

        // Lecture des headers
        for line in raw.iter().skip(1) {
            if line.is_empty() {
                break;
            }
            if let Some(value) = line.strip_prefix("Host:") {
                host = value.trim().to_string();
            } else if let Some(value) = line.strip_prefix("User-Agent:") {
                user_agent = value.trim().to_string();
            } else if let Some(value) = line.strip_prefix("Accept:") {
                accept = value.trim().to_string();
            } else if let Some(value) = line.strip_prefix("Content-Type:") {
                content_type = value.trim().to_string();
            } else if let Some(value) = line.strip_prefix("Content-Length:") {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }

        // Lecture du body brut
        let mut buffer = vec![0; content_length as usize];
        if reader.read_exact(&mut buffer).is_err() {
            return None;
        }

        let body = String::from_utf8_lossy(&buffer).to_string();

        // Start-line : méthode, path, version
        let (rtype, path, version) = match start_line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [method, path, version] => {
                let rtype = match *method {
                    "POST" => RequestType::POST,
                    "GET" => RequestType::GET,
                    "DELETE" => RequestType::DELETE,
                    _ => RequestType::UNKNOWN,
                };
                (rtype, path.to_string(), version.to_string())
            }
            _ => return None,
        };

        let content = Content::new(content_type, content_length, body);

        Some(Self {
            rtype,
            path,
            version,
            host,
            user_agent,
            accept,
            content,
        })
    }
}




pub fn parse(reader: &mut BufReader<TcpStream>) -> Vec<String> {
    let mut result = Vec::new();
    let mut request = reader.lines();

    while let Some(Ok(line)) = request.next() {
        if line.is_empty() {
            break; // Fin de requête HTTP
        }
        result.push(line);
    }

    result
}