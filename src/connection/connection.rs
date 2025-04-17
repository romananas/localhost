use std::fs;
use std::io::{BufReader, Write};
use std::net::TcpStream;

use crate::connection::multipart;
use crate::{interface, options, utils,files};
use super::{requests::Request, requests::RequestType, utils::*};

const ERROR_500: &str = "<h1>500 INTERNAL SERVER ERROR<h1>";

pub fn handle(mut stream: TcpStream, opts: options::Opts) {
    let mut reader = BufReader::new(stream.try_clone().expect("Échec de clone du stream"));

    // Parsing de la requête via la structure
    let request = match Request::parse(&mut reader) {
        Some(req) => req,
        None => {
            eprintln!("Erreur : Requête mal formée ou vide.");
            return;
        }
    };

    // println!("{:#?}", request);

    match request.rtype {
        RequestType::GET => handle_get(&request, &opts, &mut stream),
        RequestType::POST => handle_post(request, &opts, stream),
        _ => send_response(
            &mut stream,
            "HTTP/1.1 405 METHOD NOT ALLOWED",
            "text/plain",
            "Method Not Allowed",
        ),
    }
    
}

fn handle_get(request: &Request, opts: &options::Opts, stream: &mut TcpStream) {
    let path = &request.path;
    let response = get_file(path, opts);
    let status_line = get_status_line(response.status);
    let mut content_type = get_content_type(&response.file);

    let mut content = fs::read_to_string(&response.file).unwrap_or_else(|_| {
        ERROR_500.to_string()
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
                eprintln!("CGI ERROR ON GET : {}", e);
                ERROR_500.to_string()
            });
            content_type = "text/html";
        }
    }

    send_response(stream, &status_line, content_type, &content);
}


fn handle_post(request: Request, opts: &options::Opts, mut stream: TcpStream) {
    let path = &request.path;

    let response = get_file(path, opts);
    let status_line = get_status_line(response.status);
    let mut content_type = request.content._type.as_str();

    let data = request.content.data.clone();

    let mut content = fs::read_to_string(&response.file).unwrap_or_else(|_| {
        ERROR_500.to_string()
    });

    
    match content_type.trim() {
        "multipart/form-data" => {
            let datas = multipart::parse(data.clone().as_slice());
            println!("{:#?}",datas);
            for data in datas {
                match data._type {
                    multipart::Type::File(filename) => {
                        match files::write_file(opts.upload.clone(), filename, data.content) {
                            Ok(_) => {},
                            Err(e) => eprint!("{}",e) 
                        };
                    },
                    multipart::Type::Value => {},
                }
            }
        },
        _ => {},
    };
    
    // Exécution CGI si applicable
    let content = match interface::launch(path, opts, String::from("Jello")) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error : {}",e);
            content_type = "text/html";
            String::from("cgi error")
        },
    };

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
