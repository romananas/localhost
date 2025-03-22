use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn handle(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Extraction du chemin de l'URL
    let path = request_line.split_whitespace().nth(1).unwrap_or("/pages/");
    
    // Associer les chemins aux fichiers HTML
    let filename = match path {
        "/" => "index.html",
        "/hello" => "hello.html",
        _ => "404.html", // Page par défaut pour les routes inconnues
    };

    let status_line = if filename == "404.html" {
        "HTTP/1.1 404 NOT FOUND"
    } else {
        "HTTP/1.1 200 OK"
    };

    // Lire le fichier si possible
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        String::from("<h1>Erreur interne du serveur</h1>")
    });
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
