use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Write};
use std::net::TcpStream;
use uuid;

use serde_json;

use crate::connection::multipart;
use crate::utils::get_file_extention;
use crate::{interface, options, utils, files};
use super::{requests::Request, requests::RequestType, utils::*};

const ERROR_500: &[u8] = b"<h1>500 INTERNAL SERVER ERROR</h1>";

pub fn handle(mut stream: TcpStream, opts: options::Opts) {
    let mut reader = BufReader::new(stream.try_clone().expect("Échec de clone du stream"));

    let request = match Request::parse(&mut reader) {
        Some(req) => req,
        None => {
            eprintln!("Erreur : Requête mal formée ou vide.");
            return;
        }
    };

    match request.rtype {
        RequestType::GET => handle_get(&request, &opts, &mut stream),
        RequestType::POST => handle_post(request, &opts, stream),
        _ => send_response(
            &mut stream,
            "HTTP/1.1 405 METHOD NOT ALLOWED",
            "text/plain",
            b"Method Not Allowed",
        ),
    }
}

fn handle_get(request: &Request, opts: &options::Opts, stream: &mut TcpStream) {
    let path = &request.path;
    let response = get_file(path, opts);
    let status_line = get_status_line(response.status);
    let mut content_type = get_content_type(&response.file);

    let mut content: Vec<u8> = match fs::read(&response.file) {
        Ok(data) => data,
        Err(_) => ERROR_500.to_vec(),
    };

    // Exécution CGI si applicable
    if let Some(ext) = utils::get_file_extention(path) {
        if let Some(cmd) = opts.cgi_binds.get(ext) {
            let (clean_path, args) = utils::split_get_request(path).unwrap_or((path, ""));
            content = interface::exec(
                clean_path.trim_start_matches('/').to_string(),
                cmd.clone(),
                args.to_string(),
            ).map_or_else(
                |e| {
                    eprintln!("CGI ERROR ON GET : {}", e);
                    ERROR_500.to_vec()
                },
                |s| s.into_bytes(),
            );
            content_type = "text/html";
        }
    }

    send_response(stream, &status_line, content_type, &content);
}

fn handle_post(request: Request, opts: &options::Opts, mut stream: TcpStream) {
    let path = &request.path;
    let response = get_file(path, opts);
    let status_line = get_status_line(response.status);
    let rct = get_content_type(&response.file);
    let cct = request.content._type.trim();
    let data = request.content.data;

    let mut values: HashMap<String, String> = HashMap::new();

    if cct == "multipart/form-data" {
        let datas = multipart::parse(data.as_slice());
        for data in datas {
            match data._type {
                multipart::Type::File(filename) => {
                    let fname = format!("{}.{}",uuid::Uuid::new_v4().to_string(),get_file_extention(filename.as_str()).unwrap());
                    match files::write_file(opts.upload.clone(), fname.clone(), data.content) {
                        Ok(_) => {
                            values.insert(data.name, format!("{}{}", opts.upload.clone(), fname));
                        },
                        Err(e) => eprintln!("Erreur écriture fichier : {}", e),
                    };
                }
                multipart::Type::Value => {
                    let content = String::from_utf8_lossy(&data.content)
                        .split_once('\r')
                        .map(|(s, _)| s.to_string())
                        .unwrap_or_else(|| String::from_utf8_lossy(&data.content).into_owned());
                    values.insert(data.name, content);
                }
            }
        }
    }

    let values_json = serde_json::to_string(&values).unwrap_or_default();
    println!("{}", values_json);

    let content = interface::launch(path, opts, values_json)
        .map_or_else(
            |e| {
                eprintln!("CGI POST error : {}", e);
                "cgi error".to_string()
            },
            |s| s,
        )
        .into_bytes();

    send_response(&mut stream, &status_line, rct, &content);
}

fn send_response(stream: &mut TcpStream, status_line: &str, content_type: &str, body: &[u8]) {
    let header = format!(
        "{status_line}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n",
        body.len()
    );

    if let Err(e) = stream.write_all(header.as_bytes()) {
        eprintln!("Erreur envoi en-tête : {}", e);
    }

    if let Err(e) = stream.write_all(body) {
        eprintln!("Erreur envoi corps : {}", e);
    }
}
