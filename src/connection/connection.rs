use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

use crate::{interface, options, utils};
use super::{requests, utils::*};

pub fn handle(mut stream: TcpStream, opts: options::Opts) {
    let mut buf_reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    let request: Vec<String> = requests::parse(&mut buf_reader);
    
    if request.is_empty() {
        return;
    }

    let header = request[0].clone();
    let mut parts = header.split_whitespace();

    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    match method {
        "GET" => handle_get(path, &opts, &mut stream),
        "POST" => handle_post(path, &opts, request, buf_reader, stream),
        _ => send_response(&mut stream, "HTTP/1.1 405 METHOD NOT ALLOWED", "text/plain", "Method Not Allowed"),
    }
}

fn handle_get(path: &str, opts: &options::Opts, stream: &mut TcpStream) {
    let response = get_file(path, opts);
    let status_line = get_status_line(response.status);
    let mut content_type = get_content_type(&response.file);

    let mut contents = fs::read_to_string(&response.file).unwrap_or_else(|_| {
        String::from("<h1>500 INTERNAL SERVER ERROR</h1>")
    });

    let ext = match utils::get_file_extention(path) {
        Some(v) => v,
        None => "",
    };

    // call the cgi if needded
    if !ext.is_empty() && opts.cgi_binds.contains_key(ext) {
        let cmd = opts.cgi_binds.get(ext).unwrap();
        contents = match interface::exec(String::from(path.strip_prefix("/").unwrap()), cmd.clone(), String::from("")) {
            Ok(v) => v,
            Err(e) => {
                println!("{}",e);
                String::from("<h1>500 INTERNAL SERVER ERROR</h1>")
            }
        };
        content_type = "text/html"
    }

    send_response(stream, &status_line, &content_type, &contents);
}

fn handle_post(path: &str, opts: &options::Opts, request: Vec<String>, mut buf_reader: BufReader<TcpStream>, mut stream: TcpStream) {
    //* Récupérer `Content-Length`
    let mut content_length = 0;
    
    // Parcourt les lignes de l'en-tête de la requête HTTP pour trouver `Content-Length`
    for line in &request {
        if line.to_lowercase().starts_with("content-length:") {
            // Récupère la valeur après "Content-Length:" et la convertit en usize
            content_length = line.split_whitespace().nth(1).unwrap_or("0").parse::<usize>().unwrap_or(0);
        }
    }

    println!("Content-Length trouvé: {}", content_length);

    // Initialise un vecteur de bytes pour stocker le corps de la requête
    let mut body = vec![0; content_length];
    
    // Si `Content-Length` est supérieur à 0, lit exactement ce nombre d'octets dans `body`
    if content_length > 0 {
        buf_reader.read_exact(&mut body).unwrap_or(()); // ignore les erreurs éventuelles
    }

    // Convertit le contenu du body en String (UTF-8)
    let body_str = String::from_utf8_lossy(&body);
    println!("POST Body: {}", body_str);

    // Envoie une réponse HTTP 200 OK avec "received" comme contenu
    send_response(&mut stream, "HTTP/1.1 200 OK", "text/plain", "received");
}


fn send_response(stream: &mut TcpStream, status_line: &str, content_type: &str, body: &str) {
    let length = body.len();
    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{body}"
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Erreur d'écriture: {}", e);
    }
}
