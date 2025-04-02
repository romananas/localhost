use std::io::{BufRead, BufReader};
use std::net::TcpStream;

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