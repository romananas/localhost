use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use super::options;

struct Response {
    file: String,
    status: u16,
}

fn get_file(path: &str,opts: options::Opts) -> Response {
    let links = opts.links;
    match links.get(path) {
        Some(v) => return Response {file: v.clone(),status: 200},
        None => (),
    };
    Response {file: opts.not_found.clone(),status: 404}

}
    
pub fn handle(mut stream: TcpStream,opts: options::Opts) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Extraction du chemin de l'URL
    let path = request_line.split_whitespace().nth(1).unwrap_or("/");
    
    // Associer les chemins aux fichiers HTML
    let filename = get_file(path, opts);

    let status_line = if filename.status == 404 {
        "HTTP/1.1 404 NOT FOUND"
    } else {
        "HTTP/1.1 200 OK"
    };

    // Lire le fichier si possible
    let contents = fs::read_to_string(filename.file).unwrap_or_else(|_| {
        String::from("<h1>Erreur interne du serveur</h1>")
    });
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
