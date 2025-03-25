use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use super::options;

struct Answer {
    file: String,
    status: u16,
}

fn get_file(path: &str,opts: options::Opts) -> Answer {
    let links = opts.links;
    match links.get(path) {
        Some(v) => return Answer {file: v.clone(),status: 200},
        None => (),
    };
    Answer {file: opts.not_found.clone(),status: 404}

}

fn get_status_line(code: u16) -> &'static str {
    match code {
        200 => "HTTP/1.1 200 OK",
        404 => "HTTP/1.1 404 NOT FOUND",
        _ => "HTTP/1.1 500 SERVER ERRROR",
    }
}
    
pub fn handle(mut stream: TcpStream,opts: options::Opts) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Extraction du chemin de l'URL
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");
    
    // Associer les chemins aux fichiers HTML
    let answer = get_file(path, opts);

    let status_line = get_status_line(answer.status);

    // Lire le fichier si possible
    let contents = fs::read_to_string(answer.file).unwrap_or_else(|_| {
        String::from("<h1>Erreur interne du serveur</h1>")
    });
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
