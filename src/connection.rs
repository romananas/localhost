use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::path::Path;

use super::options;

struct Answer {
    file: String,
    status: u16,
}

fn get_file(path: &str, opts: &options::Opts) -> Answer {
    let links = &opts.links;
    match links.get(path) {
        Some(v) => Answer {
            file: v.clone(),
            status: 200,
        },
        None => Answer {
            file: opts.not_found.clone(),
            status: 404,
        },
    }
}

fn get_status_line(code: u16) -> &'static str {
    match code {
        200 => "HTTP/1.1 200 OK",
        201 => "HTTP/1.1 201 Created",
        400 => "HTTP/1.1 400 Bad Request",
        404 => "HTTP/1.1 404 Not Found",
        500 => "HTTP/1.1 500 Internal Server Error",
        _ => "HTTP/1.1 500 Internal Server Error",
    }
}

/// Retourne le bon `Content-Type` en fonction de l'extension du fichier
fn get_content_type(file_path: &str) -> &str {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "text/plain",
    }
}

pub fn handle(mut stream: TcpStream, opts: options::Opts) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Extraction du chemin de l'URL
    // let path = request_line.split_whitespace().nth(1).unwrap_or("/");

    // Extraction du chemin et de la méthode HTTP
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");
    
    // Récupérer le fichier associé
    let answer = get_file(path, &opts);

    let status_line = get_status_line(answer.status);
    let content_type = get_content_type(&answer.file);

    // Lire le fichier si possible
    let contents = fs::read_to_string(&answer.file).unwrap_or_else(|_| {
        String::from("<h1>500 INTERNAL SERVER ERROR</h1>")
    });
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
