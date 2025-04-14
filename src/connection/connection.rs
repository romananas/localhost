use std::fs;
use std::io::{BufReader, Read, Write};
use std::net::TcpStream;

use crate::{interface, options, utils};
use super::{requests, utils::*};

pub fn handle(mut stream: TcpStream, opts: options::Opts) {
    let mut reader = BufReader::new(stream.try_clone().expect("Échec de clone du stream"));
    let request_lines: Vec<String> = requests::parse(&mut reader);

    println!("{:#?}",request_lines);

    if request_lines.is_empty() {
        return;
    }

    let header_line = &request_lines[0];
    let mut parts = header_line.split_whitespace();

    let method = parts.next().unwrap_or("GET");
    let path = parts.next().unwrap_or("/");

    match method {
        "GET" => handle_get(path, &opts, &mut stream),
        "POST" => handle_post(path, &opts, request_lines.clone(), reader, stream),
        _ => send_response(&mut stream, "HTTP/1.1 405 METHOD NOT ALLOWED", "text/plain", "Method Not Allowed"),
    }
}

fn handle_get(path: &str, opts: &options::Opts, stream: &mut TcpStream) {
    let response = get_file(path, opts);
    let status_line = get_status_line(response.status);
    let mut content_type = get_content_type(&response.file);

    let mut content = fs::read_to_string(&response.file).unwrap_or_else(|_| {
        "<h1>500 INTERNAL SERVER ERROR</h1>".to_string()
    });

    // Exécution CGI si applicable
    if let Some(ext) = utils::get_file_extention(path) {
        if let Some(cmd) = opts.cgi_binds.get(ext) {
            let (clean_path, args) = utils::split_get_request(path).unwrap_or((path, ""));
            content = interface::exec(
                clean_path.trim_start_matches('/').to_string(),
                cmd.clone(),
                args.to_string(),
            ).unwrap_or_else(|e| {
                eprintln!("Erreur CGI : {}", e);
                "<h1>500 INTERNAL SERVER ERROR</h1>".to_string()
            });
            content_type = "text/html";
        }
    }

    send_response(stream, &status_line, content_type, &content);
}

fn handle_post(
    path: &str,
    opts: &options::Opts,
    request_lines: Vec<String>,
    mut reader: BufReader<TcpStream>,
    mut stream: TcpStream,
) {
    // Récupération du Content-Length
    let content_length = request_lines
        .iter()
        .find(|line| line.to_lowercase().starts_with("content-length:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);

    // Lecture du body
    let mut body = vec![0; content_length];
    if content_length > 0 {
        let _ = reader.read_exact(&mut body);
    }

    let body_str = String::from_utf8_lossy(&body).to_string();
    println!("Body POST : {}", body_str);

    let response = get_file(path, opts);
    let status_line = get_status_line(response.status);
    let mut content_type = get_content_type(&response.file);

    let mut content = fs::read_to_string(&response.file).unwrap_or_else(|_| {
        "<h1>500 INTERNAL SERVER ERROR</h1>".to_string()
    });

    // Exécution CGI si applicable
    if let Some(ext) = utils::get_file_extention(path) {
        if let Some(cmd) = opts.cgi_binds.get(ext) {
            content = interface::exec(
                path.trim_start_matches('/').to_string(),
                cmd.clone(),
                body_str,
            ).unwrap_or_else(|e| {
                eprintln!("Erreur CGI : {}", e);
                "<h1>500 INTERNAL SERVER ERROR</h1>".to_string()
            });
            content_type = "text/html";
        }
    }

    send_response(&mut stream, &status_line, content_type, &content);
}

fn send_response(stream: &mut TcpStream, status_line: &str, content_type: &str, body: &str) {
    let response = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Erreur d'envoi de la réponse : {}", e);
    }
}
