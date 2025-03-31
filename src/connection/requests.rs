use std::io::{BufRead, BufReader};
use std::net::TcpStream;

/// Fonction pour lire les en-têtes HTTP et récupérer le corps de la requête POST
pub fn println(reader: BufReader<&TcpStream>) {
    if let Some(Ok(t)) = reader.lines().next() {
        println!("{}", t);
    }
}

pub fn parse(reader: BufReader<&TcpStream>) -> Vec<String> {
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